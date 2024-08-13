pub fn node_cost(output_activation: f64, expected_output: f64) -> f64 {
    let loss = output_activation - expected_output;
    //println!("Loss Before Square: {loss}");
    let loss = loss.powf(2.0);
    //println!("Loss: {loss}");
    loss
}
