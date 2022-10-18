use rand::Rng;
use serde::{Deserialize, Serialize};

pub type NeuronCalculateType = f32;
type T = NeuronCalculateType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Neuron {
    pub weight: T,
    pub dendrite: Option<T>,
}

impl Neuron {
    pub fn new(weight: T, dendrite: T) -> Self {
        Self {
            weight,
            dendrite: if dendrite == 0 as T {
                Some(dendrite)
            } else {
                None
            },
        }
    }
    // Функция для создания нейрона со случайным весом
    pub fn random_new(dendrite: Option<T>) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            weight: rng.gen_range(0.0..1.0),
            dendrite,
        }
    }

    pub fn set_dendrite(&mut self, dendrite: T) {
        self.dendrite = Some(dendrite);
    }

    //Функция расчета нейрона
    pub fn axon(&mut self) -> T {
        self.dendrite.unwrap() * self.weight
    }
}
