use crate::activation::sigmoid;
use crate::matrix::Matrix;

pub struct Layer {
    weights: Matrix,
    biases: Matrix,
}

impl Layer {
    pub fn new(weights: Matrix, biases: Matrix) -> Self {
        Self { weights, biases }
    }

    pub fn forward(&self, input: &Matrix) -> Matrix {
        let weighted_input = self.weights.multiply(input);
        let z = weighted_input.add(&self.biases);

        z.map(sigmoid)
    }
}
