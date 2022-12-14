use super::neuron::{Neuron, NeuronCalculateType};
use anyhow::{anyhow, Error, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub type Layer = Vec<Arc<Neuron>>;
pub type Int = i8;

// Пока думаю создать две проекции нейронной матрицы: вертикальную и горизонтальную.
// Они будут индентичны и содержать ссылки на одни и те же нейроны.
// горизонтальная проекция нужна для передачи сигналов от нейронов.
// Вертикальная нужна для выстраивания нейронной сети.
// Можно представить как таблицу, перевёрнутую на 90 градусов.
#[derive(Debug, Serialize, Deserialize)]
pub struct Matrix {
    v_projection: Vec<Layer>,
    h_projection: Vec<Layer>,
}
trait TLayer {
    fn excite(&self) -> NeuronCalculateType;
    fn calculate(container: &mut NeuronCalculateType, val: NeuronCalculateType) {}
    fn set_dendrites(&self, val: NeuronCalculateType) {}
}

impl TLayer for Layer {
    fn excite(&self) -> NeuronCalculateType {
        let mut container: NeuronCalculateType;
        for neuron in self.iter() {
            Self::calculate(&mut container, neuron.axon());
        }
        container
    }

    fn calculate(container: &mut NeuronCalculateType, val: NeuronCalculateType) {
        *container += val;
    }

    fn set_dendrites(&self, val: NeuronCalculateType) {
        for neuron in self.iter_mut() {
            neuron.set_dendrite(val);
        }
    }
}
impl Matrix {
    pub fn run(&self) -> Result<Layer> {
        let v = self.v_projection;
        for n in 1..v.len() {
            let previous = v[n - 1];
            let current = v[n];

            current.set_dendrites(previous.excite());
        }

        Ok(*self
            .v_projection
            .last()
            .ok_or(anyhow::anyhow!("No element"))?)
    }

    // Функция для создания матрицы с случайными весами.
    pub fn cr_randomize_net(
        v_count: Int,
        h_count: Int,
        input: Vec<NeuronCalculateType>,
    ) -> Result<Matrix> {
        let mut v_projection = Self::cr_empty_projection(h_count, v_count)?;
        let mut h_projection = Self::cr_empty_projection(v_count, h_count)?;

        for i in 0..(v_count * h_count) {
            //Создаём нейрон со случайным весом
            let neuron = if i < input.len().try_into()? {
                Arc::new(Neuron::random_new(Some(
                    *input.get(i as usize).ok_or(anyhow::anyhow!("No element"))?,
                )))
            } else {
                Arc::new(Neuron::random_new(None))
            };
            // Берём первый незаполненный слой горизонтальной проекции
            let (_, h_layer) = Self::get_not_filled_layer(&mut h_projection)?
                .ok_or(anyhow!("There is no empty layer"))?;
            // Добавляем в него нейрон
            h_layer.push(neuron.clone());

            // Берём слой проекции соответствующий незаполненному слою вертикальной проекции
            let v_layer = v_projection.get_mut(h_layer.len() - 1).unwrap();
            // Добавляем в него нейрон
            v_layer.push(neuron.clone());
        }

        Ok(Matrix {
            v_projection,
            h_projection,
        })
    }
    // Функция для получения первого незаполненного слоя проекции и его индекса
    fn get_not_filled_layer(projection: &mut Vec<Layer>) -> Result<Option<(Int, &mut Layer)>> {
        for lay in 0..projection.len() {
            // Берём слой
            let layer = projection
                .get(lay)
                .ok_or(anyhow!("Error while getting layer.  index: {:?}", lay))?;
            let len = layer.len();

            // Проверяем заполнени ли вектор
            if len < layer.capacity() {
                // Если да, то возвращаем индекс и слой
                return Ok(Some((
                    len.try_into()?,
                    projection
                        .get_mut(lay)
                        .ok_or(anyhow!("Error while getting layer.  index: {:?}", lay))?,
                )));
            }
        }
        Ok(None)
    }
    // Функция для создания пустой проекции, с пустыми слоями.
    fn cr_empty_projection(size: Int, elemnt_count: Int) -> Result<Vec<Layer>> {
        let mut projection: Vec<Layer> = Vec::with_capacity(size.try_into()?);
        for _ in 1..=size {
            projection.push(Self::cr_empty_layer(elemnt_count)?);
        }
        Ok(projection)
    }

    fn cr_empty_layer(capacity: Int) -> Result<Layer> {
        Ok(Vec::with_capacity(capacity.try_into()?))
    }
}
