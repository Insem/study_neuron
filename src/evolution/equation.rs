use std::{ops::IndexMut, sync::Arc};

use crate::neuron::{
    matrix::{Layer, Matrix, TLayer},
    neuron::NeuronCalculateType,
};
use anyhow::{anyhow, bail, Error, Ok, Result};
use rand::Rng;

type EquationInt = i32;
pub type Int = i32;
#[derive(Clone)]
pub struct Population {
    individuals: Vec<Matrix>,
    count_fn: fn(&NeuronCalculateType) -> Int,
    roots: Vec<Int>,
    equals: Int,
}
impl Population {
    pub fn new(count_fn: fn(&NeuronCalculateType) -> Int, roots: Vec<Int>, equals: Int) -> Self {
        Population {
            roots,
            equals,
            individuals: Vec::new(),
            count_fn,
        }
    }

    fn is_end(
        selection: &Vec<(Int, &Matrix)>,
    ) -> std::option::Option<(Int, Vec<NeuronCalculateType>)> {
        selection.iter().find_map(|it| {
            if it.0 == 0 {
                Some((it.0, it.1.get_last_layer().unwrap().result()))
            } else {
                None
            }
        })
    }
    pub fn natural_selection(&self) -> Result<Vec<(Int, &Matrix)>> {
        println!("--start Selection");
        let mut score_arr: Vec<(Int, &Matrix)> = Vec::new();
        for individ in self.individuals.iter() {
            if !individ.is_runned() {
                println!("Matrix is not runned");
                continue;
            }
            let result: Vec<Int> = individ
                .get_last_layer()?
                .result()
                .iter()
                .map(self.count_fn)
                .collect();

            let score = self.check_result(result)?;

            score_arr.push((score, individ));
        }
        score_arr.sort_by(|a, b| a.0.cmp(&b.0));
        score_arr
            .iter()
            .for_each(|it| println!("--Sort arr {:?}", it.0));

        Ok(score_arr)
    }

    fn check_result(&self, equation_parameters: Vec<Int>) -> Result<Int> {
        let mut neuro_result: Int = 0;
        for i in 0..equation_parameters.len() {
            neuro_result += self.roots.get(i).ok_or(anyhow!(""))?
                * equation_parameters.get(i).ok_or(anyhow!("No parametr"))?;
        }

        Ok((self.equals - neuro_result).abs())
    }

    fn orgy(
        mut self: Population,
        roots: Vec<Int>,
        selection: Vec<(Int, &Matrix)>,
    ) -> Result<Population> {
        let parent_one = selection.get(0).unwrap().1;
        let parent_two = selection.get(1).unwrap().1;
        let size = parent_one.get_size();

        if size != parent_two.get_size() {
            bail!("Matrixes size is not the same");
        }
        let child = parent_one.sex(parent_two)?;
        let mut net = Matrix::cr_randomize_net(size.0, size.1, roots.clone())?;
        net.run()?;
        self.individuals.push(parent_one.clone());
        self.individuals.push(parent_two.clone());
        self.individuals.push(child);
        self.individuals.push(net);
        let mut rand = rand::thread_rng();
        loop {
            if rand.gen_range(0..1) == 1 {
                let mut net = Matrix::cr_randomize_net(size.0, size.1, roots.clone())?;
                net.run()?;
                self.individuals.push(net);
                continue;
            }
            break;
        }
        Ok(self)
    }
}

pub fn equation(
    roots: Vec<Int>,
    equals: Int,
    individuals_count: Int,
    v_size: Int,
    h_size: Int,
) -> Result<Int> {
    let mut population: Population =
        Population::new(|num| -> Int { (num).trunc() as Int }, roots.clone(), equals);

    for i in 0..individuals_count {
        let mut net = Matrix::cr_randomize_net(v_size, h_size, roots.clone())?;
        let individ: &Layer = net.run().unwrap();

        println!("--I {}, individ {:?}", i, individ.result());
        population.individuals.push(net);
    }

    let mut i = 0;
    loop {
        i += 1;
        println!("START NEW POPULATION");
        let selection = population.natural_selection()?;
        match Population::is_end(&selection) {
            Some(v) => {
                println!(
                    "END!!! POPULATION COUNT {:?}, RES {:?}",
                    i,
                    v.1.iter()
                        .map(|num| num.trunc())
                        .collect::<Vec<NeuronCalculateType>>()
                );
                return Ok(i);
            }
            None => (),
        };

        population = Population::new(|num| -> Int { (num).trunc() as Int }, roots.clone(), equals)
            .orgy(roots.clone(), selection)?;
    }
}
