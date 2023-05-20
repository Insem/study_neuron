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
    score_result: Option<Vec<EquationInt>>,
}

impl Population {
    pub fn new(
        count_fn: fn(NeuronCalculateType) -> EquationInt,
        roots: Vec<NeuronCalculateType>,
        equals: EquationInt,
    ) -> Self {
        Population {
            roots,
            equals,
            individuals: Vec::new(),
            count_fn,
            score_result: None,
        }
    }
    pub fn get_score(&mut self) -> Option<&Vec<EquationInt>> {
        self.score_result.as_ref()
    }
    pub fn natural_selection(&mut self) -> Result<Vec<(EquationInt, &Matrix)>> {
        println!("--start Selection");
        let mut score_arr: Vec<(EquationInt, &Matrix)> = Vec::new();
        let mut score_index_arr: Vec<EquationInt> = Vec::new();
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
            score_index_arr.push(score);
        }
        self.score_result = Some(score_index_arr);
        score_arr.sort_by(|a, b| a.0.cmp(&b.0));
        // score_arr
        // .iter()
        // .for_each(|it| println!("--Sort arr {:?}", it.0));

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
    let mut population_cache = _population.clone();
    let mut score_cache: Vec<EquationInt> = Vec::new();
    let mut i = 0;
    loop {
        i += 1;
        println!("START NEW POPULATION");
        let selection = population.natural_selection()?;
        let score: Vec<EquationInt> = selection.iter().map(|i| i.0).collect();
        match is_end(&selection) {
            Some(v) => {
                println!("END!!! POPULATION COUNT {:?}, RES {:?}", i, v.1);
                break;
            }
            None => (),
        };

        population = sex_population(
            roots.clone(),
            selection.clone(),
            Population::new(
                |num| -> EquationInt { (num).trunc() as EquationInt },
                roots.clone(),
                equals,
            ),
        )?;
        if i > 1 && is_bad_population(&score, &score_cache) {
            println!(
                "Population is bad, reverse. old: {:?}, new:{:?}",
                score_cache, score
            );
            population = population_cache.clone();
            continue;
        } else {
            population_cache = population.clone();
            println!("Set score_cache {:?}", score);
            score_cache = score;
        }
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

fn is_bad_population(selection: &Vec<EquationInt>, selection_old: &Vec<EquationInt>) -> bool {
    let index: i32 = selection.iter().sum();
    let old_index: i32 = selection_old.iter().sum();

    index > old_index
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
