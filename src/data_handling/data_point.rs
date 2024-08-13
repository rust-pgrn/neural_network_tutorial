use super::activation;
#[derive(Clone, Debug, Default)]
pub struct DataPoint {
    pub inputs: Vec<f64>,
    pub expected_outputs: Vec<f64>,
    pub label: usize,
}
impl DataPoint {
    pub fn new(inputs: Vec<f64>, label: usize, num_labels: usize, classify: bool) -> DataPoint {
        let expected_outputs = create_one_hot(label, num_labels, classify, &inputs);
        DataPoint {
            inputs,
            expected_outputs,
            label,
        }
    }
}
fn create_one_hot(index: usize, num: usize, classify: bool, inputs: &Vec<f64>) -> Vec<f64> {
    if classify {
        let mut one_hot = vec![0.0; num];
        //for _ in 0..num {
        //one_hot.push(0.0);
        //}
        one_hot[index] = 1.0;
        //one_hot.insert(index, 1.0);
        return one_hot;
    } else {
        let mut expected_outputs = Vec::with_capacity(inputs.len());
        for input in inputs {
            let output = input.log2();
            expected_outputs.push(output);
        }
        expected_outputs
    }
}
