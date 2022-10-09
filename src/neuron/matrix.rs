use super::neuron::{Neuron, NeuronCalculateType};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::{collections::HashMap, sync::Arc};

pub type Layer = Vec<Arc<Neuron>>;
#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix {
    v_projection: Vec<Layer>,
    h_projection: Vec<Layer>,
}

impl Matrix {
    pub fn cr_randomize_net(v_count: i32, h_count: i32) -> Matrix {
        let mut v_projection = Self::cr_empty_projection(h_count, v_count);

        //println!("--start {:?}, {:?}", v_count, h_count);
        let mut h_projection = Self::cr_empty_projection(v_count, h_count);

        for val in 0..(v_count * h_count) {
            let neuron = Arc::new(Neuron::random_new());
            //println!("--rand neuron {:?}", neuron);
            let (h_layer_number, mut h_layer) =
                Self::get_not_filled_layer(&mut h_projection).unwrap();

            h_layer.push(neuron.clone());
            let v_layer = v_projection.get_mut(h_layer.len() - 1).unwrap();
            //println!("-- H_layer {:?}, {:?}", h_layer_number as usize, h_layer);
            v_layer.push(neuron.clone());
            //println!("-- Cr layers {:?}, {:?}", v_layer, h_layer);
            //let mut v_layer = Self::get_not_filled_layer(&mut v_projection).unwrap();
            //v_layer.push(neuron.clone());
        }

        Matrix {
            v_projection,
            h_projection,
        }
    }
    fn get_not_filled_layer(projection: &mut Vec<Layer>) -> Option<(i32, &mut Layer)> {
        // for lay in projection {
        //     if lay.len() < lay.capacity() {
        //         return Some(((lay.len() - 1).try_into().unwrap(), lay));
        //     }
        // }
        for lay in 0..projection.len() {
            let layer = projection.get(lay).unwrap();
            let len = layer.len();
            //println!("--get_not_filled {:?}, {:?}, {:?}", lay, projection, len);
            if len < layer.capacity() {
                // println!("--get_not_filled for {:?}, {:?}", layer, layer.capacity());
                return Some(((len).try_into().unwrap(), projection.get_mut(lay).unwrap()));
            }
        }
        None
    }
    fn cr_empty_projection(size: i32, elemnt_count: i32) -> Vec<Layer> {
        let mut projection: Vec<Layer> = Vec::with_capacity(size as usize);
        // //println!(
        //     "--Empty vec {:?}, {:?}",
        //     projection.capacity(),
        //     size as usize
        // );
        for n in 1..=size {
            // println!(
            //     "--FOR cr_empty_projection {:?}, {:?}",
            //     n,
            //     projection.capacity()
            // );
            projection.push(Self::cr_empty_layer(elemnt_count));
        }
        // println!(
        //     "--ret cr_empty_projection {:?}, size {:?}, projection.capacity {:?}",
        //     projection,
        //     size,
        //     projection.capacity()
        // );
        projection
    }

    fn cr_empty_layer(capacity: i32) -> Layer {
        let layer: Layer = Vec::with_capacity(capacity as usize);
        //println!("--cr_empty_layer {:?}", layer);
        layer
    }
}
