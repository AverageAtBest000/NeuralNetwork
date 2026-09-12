mod matrix;
use matrix::Matrix;

fn main(){
    let matrix = Matrix::from_vector(2, 2, vec![1.0, 2.0, 3.0 ,4.0 ]);
    matrix.display();
}

