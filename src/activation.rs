use std::cmp::max_by;

pub fn activation_function(f: f64) -> f64 {
    use std::f64::consts::E;
    1.0 / (1.0 + E.powf(-f))
    //if f > 0.0 {
    //f
    //} else {
    //0.0
    //}
}
//pub enum Activation {
//    Sigmoid(Sigmoid),
//    ReLU(ReLU),
//    TanH(TanH),
//    SiLU(SiLU),
//    Softmax(Softmax),
//}
//impl Activation {
//    pub fn activate(&self, inputs: Vec<f64>, index: usize) -> f64 {
//        match self {
//            Activation::Sigmoid(sigmoid) => sigmoid.activate(inputs, index),
//            Activation::ReLU(relu) => relu.activate(inputs, index),
//            Activation::TanH(tanh) => tanh.activate(inputs, index),
//            Activation::SiLU(silu) => silu.activate(inputs, index),
//            Activation::Softmax(softmax) => softmax.activate(inputs, index),
//        }
//    }
//    pub fn derivative(&self, inputs: Vec<f64>, index: usize) -> f64 {
//        match self {
//            Activation::Sigmoid(sigmoid) => sigmoid.derivative(inputs, index),
//            Activation::ReLU(relu) => relu.derivative(inputs, index),
//            Activation::TanH(tanh) => tanh.derivative(inputs, index),
//            Activation::SiLU(silu) => silu.derivative(inputs, index),
//            Activation::Softmax(softmax) => softmax.derivative(inputs, index),
//        }
//    }
//}
//pub struct Sigmoid {}
//impl Sigmoid {
//    fn activate(&self, inputs: Vec<f64>, index: usize) -> f64 {
//        use std::f64::consts::E;
//        1.0 / (1.0 + E.powf(-inputs[index as usize]))
//    }
//    fn derivative(&self, inputs: Vec<f64>, index: usize) -> f64 {
//        let a = self.activate(inputs, index);
//        a * (1.0 - a)
//    }
//}
//pub struct ReLU {}
//impl ReLU {
//    fn activate(&self, inputs: Vec<f64>, index: usize) -> f64 {
//        let
//    }
//    fn derivative(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//}
//pub struct TanH {}
//impl TanH {
//    fn activate(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//    fn derivative(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//}
//pub struct SiLU {}
//impl SiLU {
//    fn activate(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//    fn derivative(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//}
//pub struct Softmax {}
//impl Softmax {
//    fn activate(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//    fn derivative(&self, inputs: Vec<f64>, index: usize) -> f64 {}
//}
