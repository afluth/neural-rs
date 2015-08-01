extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;

fn sigmoid(x: f32) -> f32 {
    1f32 / (1f32 + (-x).exp())
}

struct Neuron {
    output: f32,
    connections: Vec<Connection>,
}

impl Neuron {
    fn new() -> Rc<RefCell<Neuron>> {
        Rc::new(RefCell::new(Neuron {
            output: 0f32,
            connections: Vec::new(),
        }))
    }

    fn add_connection(&mut self, from: Rc<RefCell<Neuron>>) {
        let con = Connection::new(from);
        self.connections.push(con);
    }

    fn calc_output(&mut self) -> f32 {
        let mut sum = 0f32;    
    
        for con in self.connections.iter() {
            let from = con.from.borrow();
            sum += from.output * con.weight;
            //println!("weight: {} sum: {}", con.weight, sum);
        }
        
        self.output = sigmoid(sum);
        return self.output;
    }
}

struct Connection {
    weight: f32,
    from: Rc<RefCell<Neuron>>,
}

impl Connection {
    fn new(from: Rc<RefCell<Neuron>>) -> Connection {
        Connection {
            weight: rand::random::<f32>() * 2f32 - 1f32,
            from: from,
        }
    }
}

pub struct Network {
    inputs: [Rc<RefCell<Neuron>>; 9],
    hiddens: [Rc<RefCell<Neuron>>; 9],
    outputs: [Rc<RefCell<Neuron>>; 9],
}

impl Network {
    pub fn new() -> Network {
        let inputs = [
            Neuron::new(), Neuron::new(), Neuron::new(),
            Neuron::new(), Neuron::new(), Neuron::new(),
            Neuron::new(), Neuron::new(), Neuron::new(),
        ];
        
        let hiddens = [
            Neuron::new(), Neuron::new(), Neuron::new(),
            Neuron::new(), Neuron::new(), Neuron::new(),
            Neuron::new(), Neuron::new(), Neuron::new(),
        ];

        let outputs = [
            Neuron::new(), Neuron::new(), Neuron::new(),
            Neuron::new(), Neuron::new(), Neuron::new(),
            Neuron::new(), Neuron::new(), Neuron::new(),
        ];

        // Connect hiddens to the inputs
        for i in 0..hiddens.len() {
            let mut hidden = hiddens[i].borrow_mut();
     
            for j in 0..inputs.len() {
                hidden.add_connection(inputs[j].clone());
            }
        }
     
        // Connect outputs to the hiddens
        for i in 0..outputs.len() {
            let mut output = outputs[i].borrow_mut();
     
            for j in 0..hiddens.len() {
                output.add_connection(hiddens[j].clone());
            }
        }

        Network {
            inputs: inputs,
            hiddens: hiddens,
            outputs: outputs,
        }
    }

    pub fn calc(&mut self, inputs: [f32; 9]) -> [f32; 9] {
        // Set input values
        for i in 0..self.inputs.len() {
            let mut input_neuron = self.inputs[i].borrow_mut();
            input_neuron.output = inputs[i];
        }

        // Calculate hidden neuron outputs
        for i in 0..self.hiddens.len() {
            let mut hidden_neuron = self.hiddens[i].borrow_mut();
            hidden_neuron.calc_output();
        }
        
        let mut outputs = [0f32; 9];
        // Calculate output neurons
        for i in 0..self.outputs.len() {
            let mut output_neuron = self.outputs[i].borrow_mut();
            outputs[i] = output_neuron.calc_output();
        }

        return outputs;
    }
}
