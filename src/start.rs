use crate::neuron::matrix::Matrix;

use anyhow::Result;

pub fn start() -> Result<()> {
    let net = Matrix::cr_randomize_net(2, 3, vec![3.0, 4.0, 5.0])?;
    println!("--Matrix: {:?},", serde_json::to_string(&net));
    Ok(())
}
