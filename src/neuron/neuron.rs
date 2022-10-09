use rand::Rng;
use serde::{Deserialize, Serialize};

pub type NeuronCalculateType = f32;
type T = NeuronCalculateType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub weight: T,
}

impl Neuron {
    pub fn new(weight: T) -> Self {
        Self { weight }
    }
    // Функция для создания нейрона со случайным весом
    pub fn random_new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weight: rng.gen_range(0.0..1.0),
        }
    }

    //Функция расчета нейрона
    pub fn axon(&mut self, dendrite: T) -> T {
        let weight = self.weight;
        dendrite * weight
    }
}
