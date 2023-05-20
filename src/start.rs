use crate::{evolution::equation::equation, neuron::matrix::Matrix};

use anyhow::Result;

pub fn start() -> Result<()> {
    equation(vec![1.0, 2.0, 3.0, 4.0], 432, 4);
    Ok(())
}
