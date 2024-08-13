use crate::activation::activation_function;

use super::activation;
use rand::Rng;
#[derive(Debug)]
pub struct Layer {
    pub num_nodes_in: i32,
    pub num_nodes_out: i32,
    pub weights: Vec<Vec<f64>>,
    pub biases: Vec<f64>,
    pub cost_gradient_w: Vec<Vec<f64>>,
    pub cost_gradient_b: Vec<f64>,
    //TODO: Add momentum
    //TODO: Add different activations
}
impl Layer {
    pub fn new(num_nodes_in: i32, num_nodes_out: i32) -> Layer {
        let mut weights = Vec::new();
        let mut biases = Vec::new();
        let mut cost_gradient_w = Vec::new();
        let mut cost_gradient_b = Vec::with_capacity(num_nodes_out as usize);
        for i in 0..num_nodes_out {
            biases.push(0.0);
            cost_gradient_b.push(0.0);
        }
        //println!("{:?}", biases);
        for i in 0..num_nodes_in as usize {
            weights.push(Vec::new());
            cost_gradient_w.push(Vec::new());
            for j in 0..num_nodes_out {
                weights[i].push(0.0);
                cost_gradient_w[i].push(0.0)
            }
        }
        //println!("{:?}", weights);

        Layer {
            num_nodes_in,
            num_nodes_out,
            weights,
            biases,
            cost_gradient_w,
            cost_gradient_b,
        }
    }
    pub fn apply_gradients(&mut self, learn_rate: f64) {
        for node_out in 0..self.num_nodes_out as usize {
            println!("old bias: {}", self.biases[node_out]);
            println!(
                "gradientb*learn_rate={}",
                self.cost_gradient_b[node_out] * learn_rate
            );

            self.biases[node_out] -= self.cost_gradient_b[node_out] * learn_rate;

            println!("new bias: {}", self.biases[node_out]);
            println!(
                "gradientb*learn_rate={}",
                self.cost_gradient_b[node_out] * learn_rate
            );
            println!("Learn Rate: {learn_rate}");

            for node_in in 0..self.num_nodes_in as usize {
                println!("old weight: {}", self.weights[node_in][node_out]);
                println!(
                    "gradientw*learn_rate={}",
                    self.cost_gradient_w[node_in][node_out] * learn_rate
                );

                self.weights[node_in][node_out] -=
                    self.cost_gradient_w[node_in][node_out] * learn_rate;

                println!("new weight: {}", self.weights[node_in][node_out]);
                println!(
                    "gradientw*learn_rate={}",
                    self.cost_gradient_w[node_in][node_out] * learn_rate
                );
            }
        }
    }
    pub fn calculate_outputs(&self, inputs: &Vec<f64>) -> Vec<f64> {
        //println!("Calculating Individual Layer Outputs!");
        let mut weighted_inputs: Vec<f64> = Vec::new();
        for (node_out, bias) in self.biases.iter().enumerate() {
            //println!("First Loop");
            let mut weighted_input = *bias;
            for (node_in, _) in self.weights.iter().enumerate() {
                //println!("Second Loop");
                //println!("{weighted_input}");
                weighted_input += inputs[node_in] * self.weights[node_in][node_out];
                //println!("{weighted_input}");
            }
            weighted_inputs.push(weighted_input);
            //)
        }
        let mut activations = Vec::with_capacity(self.num_nodes_out as usize);
        for node_out in 0..self.num_nodes_out {
            activations.push(activation_function(weighted_inputs[node_out as usize]));
        }
        println!("weighted_inputs: {:#?}", weighted_inputs);
        println!("activations: {:#?}", activations);

        activations
    }
    pub fn initialize_random_weights(&mut self) {
        for node_out in 0..self.num_nodes_out as usize {
            self.biases[node_out] = 0.0;
            for node_in in 0..self.num_nodes_in as usize {
                let random_value: f64 = rand::thread_rng().gen();
                self.weights[node_in][node_out] = random_value / (self.num_nodes_in as f64).sqrt();
            }
        }
    }
}
#[cfg(test)]
mod tests {}
