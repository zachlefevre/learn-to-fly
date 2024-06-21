use rand::{Rng, RngCore};

struct LayerTopology {
    neurons: i32
}

pub struct Network {
    layers: Vec<Layer>
}


impl Network {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers.iter().fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    fn random(rng: &mut dyn RngCore, layers: &[LayerTopology]) -> Network {
        Self {layers: layers.windows(2).map(|layers| Layer::random(rng, layers[0].neurons, layers[1].neurons)).collect()}
    }
}


struct Layer {
    neurons: Vec<Neuron>
}

impl Layer {
    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|neuron| neuron.propagate(&inputs)).collect::<Vec<_>>()
    }

    fn random(rng: &mut dyn RngCore, input_size: i32, output_size: i32) -> Self {
        Self {neurons: (0..output_size).map(|_| Neuron::random(rng, input_size)).collect()}
    }
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>
}

impl Neuron {
    fn propagate(&self, inputs: &[f32]) -> f32 {
        assert_eq!(inputs.len(), self.weights.len());
        let output: f32 = self.weights.iter()
            .zip(inputs).map(|(weight, input)| weight * input)
            .sum();
        (output + self.bias).max(0.0)
    }

    fn random(rng: &mut dyn RngCore, input_size: i32) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);
        let weights = (0..input_size).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Neuron {
            bias,
            weights
        }
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    mod NeuronTest {
        use crate::Neuron;
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn propagate() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8, 0.9],
            };

            // Ensures `.max()` (our ReLU) works:
            assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0, -10.0]),
                0.0,
            );

            // `0.5` and `1.0` chosen by a fair dice roll:
            assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0, 2.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + (0.9 * 2.0) + 0.5,
            );


        }}
}
