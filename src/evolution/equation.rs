use std::sync::Arc;

use crate::neuron::{
    matrix::{Layer, Matrix, TLayer},
    neuron::NeuronCalculateType,
};
use anyhow::{anyhow, Error, Ok, Result};

type EquationInt = i32;
#[derive(Clone)]
pub struct Population {
    individuals: Vec<Matrix>,
    count_fn: fn(NeuronCalculateType) -> EquationInt,
    roots: Vec<NeuronCalculateType>,
    equals: EquationInt,
}

impl Population {
    pub fn new(
        count_fn: fn(NeuronC
            alculateType) -> EquationInt,
        roots: Vec<NeuronCalculateType>,
        equals: EquationInt,
    ) -> Self {
        Population {
            roots,
            equals,
            individuals: Vec::new(),
            count_fn,
        }
    }

    pub fn natural_selection(&self) -> Result<Vec<(EquationInt, &Matrix)>> {
        println!("--start Selection");
        let mut score_arr: Vec<(EquationInt, &Matrix)> = Vec::new();
        for individ in self.individuals.iter() {
            if !individ.is_runned() {
                println!("Matrix is not runned");
                continue;
            }
            let lay = individ.get_last_layer()?.result();
            let a = (self.count_fn)(*lay.get(0).unwrap());
            let b = (self.count_fn)(*lay.get(1).unwrap());
            let c = (self.count_fn)(*lay.get(2).unwrap());
            let d = (self.count_fn)(*lay.get(3).unwrap());
            //println!("")
            let score = self.check_result(a, b, c, d);

            score_arr.push((score, individ));
        }
        score_arr.sort_by(|a, b| a.0.cmp(&b.0));
        score_arr
            .iter()
            .for_each(|it| println!("--Sort arr {:?}", it.0));

        Ok(score_arr)
    }

    pub fn check_result(
        &self,
        a: EquationInt,
        b: EquationInt,
        c: EquationInt,
        d: EquationInt,
    ) -> EquationInt {
        let neuro_result = 1 * a + 2 * b + 3 * c + 4 * d;
        // println!(
        //     "--Calc a: {}, b:{}, c:{}, d:{}, res {}, res_fn {}",
        //     a,
        //     b,
        //     c,
        //     d,
        //     neuro_result,
        //     (self.equals - neuro_result).abs()
        // );
        (self.equals - neuro_result).abs()
    }
}

pub fn equation(
    roots: Vec<NeuronCalculateType>,
    equals: EquationInt,
    individuals_count: EquationInt,
) -> Result<()> {
    let mut _population: Population = Population::new(
        |num| -> EquationInt { (num).trunc() as EquationInt },
        roots.clone(),
        equals,
    );

    for i in 0..individuals_count {
        let mut net = Matrix::cr_randomize_net(4, 4, roots.clone())?;
        let individ: &Layer = net.run().unwrap();

        println!("--I {}, individ {:?}", i, individ.result());
        _population.individuals.push(net);
    }

    let mut population = _population.clone();
    let mut i = 0;
    loop {
        i += 1;
        println!("START NEW POPULATION");
        let selection = population.natural_selection()?;
        match is_end(&selection) {
            Some(v) => {
                println!(
                    "END!!! POPULATION COUNT {:?}, RES {:?}",
                    i,
                    v.1.iter().map(|num| num.trunc()).collect::<Vec<f32>>()
                );
                break;
            }
            None => (),
        };

        population = sex_population(
            roots.clone(),
            selection,
            Population::new(
                |num| -> EquationInt { (num).trunc() as EquationInt },
                roots.clone(),
                equals,
            ),
        )?;
    }

    Ok(())
}

fn is_end(selection: &Vec<(EquationInt, &Matrix)>) -> std::option::Option<(i32, Vec<f32>)> {
    selection.iter().find_map(|it| {
        if it.0 == 0 {
            Some((it.0, it.1.get_last_layer().unwrap().result()))
        } else {
            None
        }
    })
}

fn sex_population(
    roots: Vec<NeuronCalculateType>,
    selection: Vec<(EquationInt, &Matrix)>,
    mut population: Population,
) -> Result<Population> {
    let parent_one = selection.get(0).unwrap().1;
    let parent_two = selection.get(1).unwrap().1;
    let child = parent_one.sex(parent_two)?;
    let mut net = Matrix::cr_randomize_net(4, 4, roots.clone())?;
    net.run();
    population.individuals.push(parent_one.clone());
    population.individuals.push(parent_two.clone());
    population.individuals.push(child);
    population.individuals.push(net);
    Ok(population)
}
