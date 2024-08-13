use data_point::DataPoint;
use neural_network::NeuralNetwork;
use rand::{self, seq::SliceRandom};
mod activation;
mod cost;
#[path = "./data_handling/data_point.rs"]
mod data_point;
#[path = "./data/iris/iris.rs"]
mod iris;
mod layer;
mod neural_network;
fn main() {
    let mut neural_network = NeuralNetwork::new(&[4, 4, 3, 3]);
    ////println!("Going to run Cost function!");
    for layer in &mut neural_network.layers {
        layer.initialize_random_weights();
    }
    println!("{:#?}", neural_network);
    println!("\n\n\n\n\n");
    let training_data = get_data();
    let cost = neural_network.cost_of_all(&training_data);

    //println!("Cost before learning: {cost}");
    for point in &training_data {
        //println!(
        //"outputs: {:?}",
        //neural_network.calculate_outputs(&point.inputs)
        //);
        //println!("expected outputs: {:?}", point.expected_outputs);
    }

    neural_network.learn(&training_data, 0.0001);
    println!("{:#?}", neural_network);
    println!("\n\n\n\n\n");

    let new_cost = neural_network.cost_of_all(&training_data);
    println!("Old cost: {cost}");
    println!("Cost after learning: {new_cost}");
    for point in training_data {
        println!(
            "outputs: {:?}",
            neural_network.calculate_outputs(&point.inputs)
        );
        println!("expected outputs: {:?}", point.expected_outputs);
    }
    println!("Old cost: {cost}");
    println!("Cost after learning: {new_cost}");
}
fn get_data() -> Vec<DataPoint> {
    let v = iris::get_data();
    let mut data = Vec::with_capacity(152);
    for point in v {
        let inputs = &point[0..=3];
        let label = *point.last().unwrap() as usize;
        data.push(DataPoint::new(inputs.to_vec(), label, 3, true))
    }

    let mut rng = rand::thread_rng();
    data.shuffle(&mut rng);
    println!("{:#?}", data);
    data
}
