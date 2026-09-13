mod matrix;
mod activation;

use matrix::Matrix;
use activation::sigmoid;

fn main(){
    let mut matrix = Matrix::from_vector(2, 2, vec![-1.0, 0.0, 1.0, 2.0]);

    let activated = matrix.map(sigmoid);
    
    activated.display();
}

