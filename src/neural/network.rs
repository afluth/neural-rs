use rand::{self, thread_rng, Rng};

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Network {
    pub num_inputs: usize,
    pub layers: Vec<Layer>,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Layer {
    pub bias: f32,
    pub neurons: Vec<Neuron>,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Neuron {
    pub weights: Vec<f32>,
}

impl Network {
    
    /// Constructs a new neural network with the specified dimensions
    pub fn with_dimensions(dimensions: &[usize]) -> Network {
        assert!(dimensions.len() > 2);
        
        let mut rng = thread_rng();
        let mut layers = Vec::with_capacity(dimensions.len() - 1);
        
        let mut prev_size = dimensions[0];
        for &size in dimensions[1..].iter() {
            let bias = rng.gen::<f32>() * 2f32 - 1f32;
            let mut neurons = Vec::with_capacity(size);
            
            for _ in 0..size {
                let mut weights = Vec::with_capacity(prev_size);
                
                for _ in 0..prev_size {
                    weights.push(rng.gen::<f32>() * 2f32 - 1f32);
                }
                
                neurons.push(Neuron {
                    weights: weights,
                });
            }
            
            layers.push(Layer {
                bias: bias,
                neurons: neurons,
            });
            
            prev_size = size;
        }
        
        Network {
            num_inputs: dimensions[0],
            layers: layers,
        }
    }
    
    pub fn reproduce(&self, partner: &Network) -> Network {
        let mut rng = rand::thread_rng();
        
        assert_eq!(self.layers.len(), partner.layers.len());
        let layers = self.layers.iter()
            .zip(&partner.layers)
            .map(|(self_layer, partner_layer)| {
                
                assert_eq!(self_layer.neurons.len(), partner_layer.neurons.len());
                let neurons = self_layer.neurons.iter()
                    .zip(&partner_layer.neurons)
                    .map(|(self_neuron, partner_neuron)| {
                        
                        assert_eq!(self_neuron.weights.len(), partner_neuron.weights.len());
                        let weights = self_neuron.weights.iter()
                            .zip(&partner_neuron.weights)
                            .map(|(&a, &b)| {
                                if rng.gen_weighted_bool(50) {
                                    rng.gen::<f32>() * 2f32 - 1f32
                                } 
                                else if rng.gen() { a } else { b }
                            })
                            .collect();
                            
                        Neuron {
                            weights: weights,
                        }
                    })
                    .collect();
                
                Layer {
                    bias: if rng.gen() { self_layer.bias } else { partner_layer.bias },
                    neurons: neurons,
                }
            })
            .collect();
        
        Network {
            num_inputs: self.num_inputs,
            layers: layers,
        }
    }
    
    // Forward Propagation
    pub fn run(&mut self, inputs: &[f32]) -> Vec<Vec<f32>> {
        let mut outputs = Vec::new();
        let mut prev_layer_out = Vec::from(inputs);
        
        for layer in &mut self.layers {
            let mut layer_out = Vec::new();
            
            for neuron in &mut layer.neurons {
                let mut sum = layer.bias;
                
                for (weight, input) in &mut neuron.weights.iter().zip(&prev_layer_out) {
                    sum += input * weight;
                }
                
                layer_out.push(sigmoid(sum))
            }
            outputs.push(prev_layer_out);
            prev_layer_out = layer_out;
        }
        
        outputs.push(prev_layer_out);
        
        outputs
    }
    
}

pub fn sigmoid(x: f32) -> f32 {
    1f32 / (1f32 + (-x).exp())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sigmoid() {
        assert_eq!(0.62245935, sigmoid(0.5));
        assert_eq!(0.37754068, sigmoid(-0.5));
    }

    #[test]
    fn test_with_dimensions() {
        let mut net = Network::with_dimensions(&[2, 3, 2]);

        let output = net.run(&[1.0, 0.0]);

        assert_eq!(3, output.len());
        assert_eq!(2, output[0].len());
        assert_eq!(3, output[1].len());
        assert_eq!(2, output[2].len());
    }

    // https://mattmazur.com/2015/03/17/a-step-by-step-backpropagation-example/
    #[test]
    fn test_example() {
        let mut net = Network {
            num_inputs: 2,
            layers: vec![
                Layer {
                    bias: 0.35,
                    neurons: vec![
                        Neuron {
                            weights: vec![0.15, 0.2],
                        },
                        Neuron {
                            weights: vec![0.25, 0.3],
                        },
                    ],
                },
                Layer {
                    bias: 0.60,
                    neurons: vec![
                        Neuron {
                            weights: vec![0.4, 0.45],
                        },
                        Neuron {
                            weights: vec![0.5, 0.55],
                        },
                    ],
                },
            ],
        };
 
        let output = net.run(&[0.05, 0.1]);

        // Inputs
        assert_eq!(0.05, output[0][0]);
        assert_eq!(0.1, output[0][1]);

        // Hidden layer output
        assert_eq!(0.59327, output[1][0]);
        assert_eq!(0.59688437, output[1][1]);
        
        // Output layer output
        assert_eq!(0.75136507, output[2][0]);
        assert_eq!(0.7729285, output[2][1]);
    }
} 