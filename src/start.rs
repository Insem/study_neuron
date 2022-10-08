use crate::neuron::{matrix::Matrix, neuron::Neuron};

pub fn start() {
    let net = Matrix::cr_randomize_net(2, 5);
    println!("--Matrix: {:?},", net)
}
