use crate::neuron::{matrix::Matrix, neuron::Neuron};

use anyhow::{anyhow, Error, Result};

pub fn start() -> Result<()> {
    let net = Matrix::cr_randomize_net(2, 3)?;
    println!("--Matrix: {:?},", serde_json::to_string(&net));
    Ok(())
}
