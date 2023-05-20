use std::{
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

use crate::{evolution::equation::equation, neuron::matrix::Matrix};

use anyhow::Result;
use rand::Rng;

pub fn start() -> Result<()> {
    let mut fw = OpenOptions::new()
        .append(true)
        .create(true) // Optionally create the file if it doesn't already exist
        .open("./compare.csv")
        .expect("Unable to open file");
    //equation(vec![1.0, 2.0, 3.0, 4.0], 231, 4);
    let fr = File::open("./compare.csv")?;

    for _l in BufReader::new(fr).lines() {
        let line = _l?;
        let sep = line.find(";").unwrap();
        let num: i32 = (&line[..sep]).parse()?;
        let old_ans: i32 = (&line[sep + 1..]).parse()?;
        println!("line {:?}, {:?}, {:?}", line, num, old_ans);

        //let ans = equation(vec![1.0, 2.0, 3.0, 4.0], 231, 4)?;

        // f.write_all(format!("{};{}\n", random, ans).as_bytes())
        //     .expect("Unable to write file");
        // println!("Start equation with {:?}", random);
    }
    Ok(())
}
