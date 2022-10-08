use rand::Rng;

pub type NeuronCalculateType = f32;
type T = NeuronCalculateType;

#[derive(Debug)]
pub struct Neuron {
    pub weight: T,
}

impl Neuron {
    pub fn new(weight: T) -> Self {
        Self { weight }
    }
    pub fn random_new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weight: rng.gen_range(0.0..1.0),
        }
    }
    pub fn axon(&mut self, dendrite: T) -> T {
        let weight = self.weight;
        dendrite * weight
    }
}
