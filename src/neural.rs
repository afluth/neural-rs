use rand::{self, thread_rng, Rng};

const NUM_NEURONS: usize = 9;
const NUM_WEIGHTS: usize = 81 * 2;

fn sigmoid(x: f32) -> f32 {
    1f32 / (1f32 + (-x).exp())
}

#[derive(RustcEncodable, RustcDecodable)]
pub struct Network {
    weights: Vec<f32>,
}

impl Network {
    pub fn new() -> Network {
        let weights = thread_rng().gen_iter::<f32>()
                .map(|w| w * 2f32 - 1f32)
                .take(NUM_WEIGHTS)
                .collect();

        Network::from_weights(weights)
    }

    pub fn from_weights(weights: Vec<f32>) -> Network {
        Network {
            weights: weights,
        }
    }

    pub fn calc(&self, mut inputs: [f32; NUM_NEURONS]) -> [f32; NUM_NEURONS] {
        let mut outputs = [0f32; NUM_NEURONS];        

        for layer in self.weights.chunks(NUM_NEURONS.pow(2)) {

            calc_layer(&inputs, layer, &mut outputs);

            // outputs from this layer are inputs for the next
            inputs = outputs;
        }

        return outputs;
    }
    
    pub fn reproduce(&self, partner: &Network) -> Network {
        let mut rng = rand::thread_rng();

        let child_weights = self.weights.iter()
                .zip(partner.weights.iter())
                .map(|(&a, &b)| {
                    // A 1 in 100 chance of mutation occuring
                    if rng.gen_weighted_bool(50) { 
                        rng.gen::<f32>() * 2f32 - 1f32
                    }
                    else if rand::random::<bool>() { a } else { b }
                })
                .collect();

        return Network::from_weights(child_weights);
    }
}

fn calc_layer(inputs: &[f32], layer: &[f32], outputs: &mut [f32]) {
    for (connections, output) in layer.chunks(NUM_NEURONS)
                                      .zip(outputs.iter_mut()) {

        *output = calc_neuron(&inputs, connections);
    }
}

fn calc_neuron(inputs: &[f32], connections: &[f32]) -> f32 {
    let mut sum = 0f32;    
    for (weight, input) in connections.iter()
                                      .zip(inputs.iter()) {
        sum += input * weight;
    }
    return sigmoid(sum);
}

#[cfg(test)]
mod tests {
    use super::sigmoid;
    
    #[test]
    fn test_sigmoid() {
        assert_eq!(0, 0);
        assert_eq!(0.62245935, sigmoid(0.5));
        assert_eq!(0.37754068, sigmoid(-0.5));
    }
}
