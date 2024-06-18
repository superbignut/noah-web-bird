use rand::{Rng, RngCore};

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug, PartialEq)]
struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

#[derive(Debug)]
pub struct LayerTopology {
    pub neurons: usize,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layer| Layer::random(rng, layer[0].neurons, layer[1].neurons))
            .collect();
           
        Self { layers }
        
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }


    fn random(rng: &mut dyn RngCore, input_size:usize, output_size: usize) -> Self{

        let neurons = (0..input_size)
            .map(|_| Neuron::random(rng, input_size))
            .collect();
        
        Self { neurons }
    }
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        // 每个神经元的输入都是上一层的全部输出，全连接的话
        assert_eq!(inputs.len(), self.weights.len());

        let mut output: f32 = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum();

        (self.bias + output).max(0.0)
    }

    fn random(rng: &mut dyn RngCore, input_size: usize) -> Self {
        
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..input_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        Self{ bias, weights}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use approx::assert_relative_eq;

    #[test]
    fn it_works() {
        let s1 = vec![1, 2, 3];
        let s2 = vec![4, 5, 6];

        let mut iter = s1.iter().zip(&s2);

        println!("{:#?}", s1);
    }


    #[test]
    fn random(){
        let mut rng = ChaCha8Rng::from_seed(Default::default());

        let neuron1 = Neuron::random(&mut rng, 4);        

        assert_relative_eq!(neuron1.bias, -0.6255188);
        assert_relative_eq!(neuron1.weights.as_slice(), [0.67383957, 0.8181262, 0.26284897, 0.5238807].as_slice());
    }

    #[test]
    fn progagate(){
        let neuron = Neuron{
            bias : 0.5,
            weights : vec![-0.3, 0.8],
        };

        assert_relative_eq!(neuron.propagate(&[-10.0, -8.0]), 0.0);
        assert_relative_eq!(neuron.propagate(&[0.5, 1.0]), 1.15);
    }

}
