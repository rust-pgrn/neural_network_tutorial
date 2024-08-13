pub fn activation_function(f: f64) -> f64 {
    use std::f64::consts::E;
    1.0 / (1.0 + E.powf(-f))
}
