use crate::neuron::matrix::Matrix;
use anyhow::{anyhow, Error, Result};

type EquationInt = i8;

pub fn calc(roots: Vec<EquationInt>, equal: EquationInt) -> Result<()> {
    let net = Matrix::cr_randomize_net(3, 4, vec![3.0,6.0,23.0, 9.0])?;
    let equation = net.run();
    Ok(())
}
