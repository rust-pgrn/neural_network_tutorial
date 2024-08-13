use super::cost;
use crate::{
    data_point::{self, DataPoint},
    layer::Layer,
};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}
impl NeuralNetwork {
    pub fn new(layer_sizes: &[i32]) -> NeuralNetwork {
        let mut layers = Vec::with_capacity(layer_sizes.len() - 1);
        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i + 1]));
        }
        NeuralNetwork { layers }
    }
    pub fn calculate_outputs(&self, inputs: &Vec<f64>) -> Vec<f64> {
        //println!("Inputs: {:?}", inputs);
        let mut weighted_inputs = inputs.clone();
        //println!("Cloned Inputs: {:?}", weighted_inputs);
        for layer in self.layers.iter() {
            //println!("Old Inputs: {:?}", weighted_inputs);
            weighted_inputs = layer.calculate_outputs(&weighted_inputs);
            //println!("New Inputs: {:?}", weighted_inputs);
        }
        weighted_inputs
    }
    pub fn classify(&self, inputs: Vec<f64>) -> usize {
        let outputs = self.calculate_outputs(&inputs);
        let index_of_max: Option<usize> = outputs
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(index, _)| index);
        index_of_max.unwrap()
    }
    pub fn cost(&self, data_point: &DataPoint) -> f64 {
        //println!("Cost function starting!");
        let outputs = self.calculate_outputs(&data_point.inputs);
        //println!("Calculated Outputs!");
        //println!("Outputs: {:?}", outputs);
        //println!("Expected Outputs: {:?}", data_point.expected_outputs);
        let mut cost = 0.0;
        for (node_out, output) in outputs.iter().enumerate() {
            //println!("Cost Before: {cost}");
            //println!(
            //"output: {} expected_output: {}",
            //output, data_point.expected_outputs[node_out]
            //);
            cost += cost::node_cost(*output, data_point.expected_outputs[node_out]);
            //println!("Cost After: {cost}");
        }
        cost
    }
    pub fn cost_of_all(&self, data: &Vec<DataPoint>) -> f64 {
        let mut total_cost = 0.0;
        for data_point in data {
            total_cost += self.cost(data_point)
        }

        total_cost / data.len() as f64
    }
    pub fn learn(&mut self, training_data: &Vec<DataPoint>, learn_rate: f64) {
        let h = 0.0001;
        let original_cost = self.cost_of_all(&training_data);
        for i in 0..self.layers.len() {
            for node_in in 0..self.layers[i].num_nodes_in as usize {
                for node_out in 0..self.layers[i].num_nodes_out as usize {
                    self.layers[i].weights[node_in][node_out] += h;

                    let delta_cost = self.cost_of_all(&training_data) - original_cost;
                    self.layers[i].weights[node_in][node_out] -= h;
                    self.layers[i].cost_gradient_w[node_in][node_out] = delta_cost / h;
                    println!(
                        "gradient w: {}",
                        self.layers[i].cost_gradient_w[node_in][node_out]
                    );
                }
            }
            for j in 0..self.layers[i].biases.len() {
                self.layers[i].biases[j] += h;
                let delta_cost = self.cost_of_all(&training_data);
                self.layers[i].biases[j] -= h;
                self.layers[i].cost_gradient_b[j] = delta_cost / h;
                println!("gradients of b: {:?}", self.layers[i].cost_gradient_b);
                println!("gradient b: {}", self.layers[i].cost_gradient_b[j]);
            }
        }
        for layer in &mut self.layers {
            println!("old weight: {:?}", layer.weights);
            //println!("old biases: {:?}", layer.biases);
            layer.apply_gradients(learn_rate);
            println!("new weight: {:?}", layer.weights);
            //println!("new biases: {:?}", layer.biases);
        }
    }
}
