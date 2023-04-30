use std::ops::{Deref, DerefMut};

use rand::Rng;
use serde::{Deserialize, Serialize};

pub type NeuronCalculateType = f32;
type T = NeuronCalculateType;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Neuron {
    pub weight: T,
    pub dendrite: Option<T>,
}

// impl Deref for Neuron {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.weight
//     }
// }
// impl DerefMut for Neuron {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.weight
//     }
// }
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

    // pub fn random(&self) {
    //     let mut rng = rand::thread_rng();
    //     self.weight = rng.gen_range(0.0..1.0);
    //     self.dendrite = Some(rng.gen_range(0.0..1.0));
    // }

    pub fn set_dendrite(&mut self, dendrite: T) {
        self.dendrite = Some(dendrite);
        //println!("--Dendrite {}", dendrite);
    }

    //Функция расчета нейрона
    pub fn axon(&self) -> T {
        let dendr = self.dendrite.unwrap();
        //println!("--Axon {}, {}, {}", dendr, self.weight, dendr * self.weight);
        dendr * self.weight
    }
}
