use crate::neuron::matrix::Matrix;
use anyhow::{anyhow, Error, Result};

type EquationInt = i8;

pub fn calc(roots: Vec<EquationInt>, equal: EquationInt) -> Result<()> {
    let net = Matrix::cr_randomize_net(3, 4)?;
    net.run();
    Ok(())
}
