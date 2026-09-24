mod layer;
mod matrix;
mod activation;
use crate::layer::Layer;
use crate::matrix::Matrix;
use crate::activation::sigmoid;

fn main() {
    let input = Matrix::from_vector(2, 1, vec![2.0, 3.0]);
    let weights = Matrix::from_vector(3,2, vec![ 1.0, 0.0,  0.0, 1.0,  1.0, 1.0, ], );
    let biases = Matrix::from_vector(3, 1, vec![0.0, 0.0, -4.0]);

    let layer = Layer::new(weights, biases);
    let output = layer.forward(&input);

    output.display();
}
