use crate::neuron::{matrix::Matrix, neuron::NeuronCalculateType};
use anyhow::{anyhow, Error, Result};

type EquationInt = i8;

pub struct Population {
    individuals: Vec<Matrix>,
}

impl Population {
    pub fn new() -> Self {
        Population {
            individuals: Vec::new(),
        }
    }
}

pub fn equation(roots: Vec<NeuronCalculateType>, individuals_count: EquationInt) -> Result<()> {
    let population = Population::new();

    for i in 1..individuals_count {
        let net = Matrix::cr_randomize_net(3, 4, roots.try_into()?)?;
        net.run();
        population.individuals.push(net);
    }
    Ok(())
}
