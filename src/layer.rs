mod matrix;
use matrix::Matrix;

pub struct Layer{
    weights: Matrix,
    biases: Matrix,
}

impl Layer{

    pub fn new(neurons: Matrix, weights: Matrix) -> Layer{
        assert_eq!(weights.rows, weights.cols);
        assert_eq!(neurons.rows, weights.cols);

        Matrix::multiply(weights, neurons);

    }


}
