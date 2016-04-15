
pub struct Network {
    num_inputs: u32,
    layers: Vec<Layer>,
}

struct Layer {
    nodes: Vec<Neuron>,
}

struct Neuron {
    weights: Vec<f32>,
}

impl Network {
    
    pub fn with_dimensions(dimensions: &[u32]) -> Network {
        
        let mut layers = Vec::with_capacity(dimensions.len() - 1);
        

    }
    
}