use crate::{evolution::equation::equation, neuron::matrix::Matrix};

use anyhow::Result;

pub fn start() -> Result<()> {
    equation(vec![3.0, 5.0, 6.0], 70, 4);
    Ok(())
}
