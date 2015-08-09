extern crate rand;

use std::cell::RefCell;
use std::rc::Rc;

const NUM_LAYERS: usize = 3;
const NUM_NEURONS: usize = 9;

fn sigmoid(x: f32) -> f32 {
    1f32 / (1f32 + (-x).exp())
}

struct Neuron {
    output: f32,
    connections: Vec<Connection>,
}

impl Neuron {
    fn new() -> Neuron {
        Neuron {
            output: 0f32,
            connections: Vec::with_capacity(NUM_NEURONS),
        }
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
    layers: Vec<Vec<Rc<RefCell<Neuron>>>>,
}

impl Network {
    pub fn new() -> Network {
        
        // Populate layers of neurons
        let mut layers: Vec<Vec<Rc<RefCell<Neuron>>>> = 
                Vec::with_capacity(NUM_LAYERS);
        
        for i in 0..NUM_LAYERS {
            
            let mut neurons = Vec::with_capacity(NUM_NEURONS);
            for _ in 0..NUM_NEURONS {
                
                let mut neuron = Neuron::new();
                
                // Interconnect the layers
                if i > 0 {
                    for input in layers[i-1].iter() {
                        neuron.add_connection(input.clone());
                    }
                }

                neurons.push(Rc::new(RefCell::new(neuron)));
            }
            layers.push(neurons);
        }
        
        Network {
            layers: layers,
        }
    }

    pub fn calc(&mut self, inputs: [f32; NUM_NEURONS]) -> [f32; NUM_NEURONS] {
        let mut outputs = [0f32; 9];
        
        for (i, neurons) in self.layers.iter().enumerate() {
            for (j, neuron_cell) in neurons.iter().enumerate() {
                let mut neuron = neuron_cell.borrow_mut();
                
                if i == 0 {
                    // Set input values
                    neuron.output = inputs[j];
                } else {
                    // Calculate neurons
                    neuron.calc_output();
                    
                    // Capture outputs
                    if i == NUM_LAYERS - 1 {
                        outputs[j] = neuron.output;
                    }
                }
            }
        }

        return outputs;
    }

    pub fn reproduce(&self, partner: &Network) -> Network {
        return Network::new();
    }
}
