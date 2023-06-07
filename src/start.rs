use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::{evolution::equation::equation, neuron::matrix::Matrix};

use anyhow::Result;
use rand::Rng;

pub fn start() -> Result<()> {
    equation(vec![1, 2, 3, 5], 431, 40, 4, 4)?;
    // let mut f = OpenOptions::new()
    //     .append(true)
    //     .create(true) // Optionally create the file if it doesn't already exist
    //     .open("./compare.csv")
    //     .expect("Unable to open file");
    //equation(vec![1.0, 2.0, 3.0, 4.0], 231, 4);
    // for i in 1..10000 {
    //     let random = rand::thread_rng().gen_range(0..10000);
    //     let ans = equation(vec![1.0, 2.0, 3.0, 4.0], 231, 4, 4, 4)?;

    //     f.write_all(format!("{};{}\n", random, ans).as_bytes())
    //         .expect("Unable to write file");
    //     println!("Start equation with {:?}", random);
    // }
    Ok(())
}
