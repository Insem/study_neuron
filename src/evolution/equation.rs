use crate::neuron::{
    matrix::{Layer, Matrix},
    neuron::NeuronCalculateType,
};
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

// fn CastNet(lay: Layer) -> Result<EquationInt> {
//     let mut cast: NeuronCalculateType = 1.0;
//     for v in lay {
//         cast *= v.axon();
//     }
//     Ok((cast * 100.0).trunc() as EquationInt)
// }

pub fn equation(
    roots: Vec<NeuronCalculateType>,
    equals: EquationInt,
    individuals_count: EquationInt,
) -> Result<()> {
    let population = Population::new();

    // for i in 1..individuals_count {
    //     let net = Matrix::cr_randomize_net(3, 4, roots.try_into()?)?;
    //     net.run();
    //     population.individuals.push(net);
    // }

    // loop {
    //     let diffs: Vec<(EquationInt, &Matrix)> = Vec::new();
    //     for matrix in population.individuals {
    //         let diff = CastNet(matrix.get_last_layer()?)? - equals;
    //         diffs.push((diff, &matrix));
    //     }
    //     diffs.sort_by(|a, b| b.age.cmp(&a.age));
    // }

    Ok(())
}
