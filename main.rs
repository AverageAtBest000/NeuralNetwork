mod matrix;
use matrix::Matrix;

fn main(){
    let mut matrix_1 = Matrix::from_vector(3, 2, vec![1.0, 2.0, 3.0 ,4.0, 5.0, 6.0 ]);
    let mut matrix_2 = Matrix::from_vector(2, 2, vec![1.0, 2.0, 3.0 ,4.0 ]);
    matrix_1.display();
    println!();
    matrix_2.display();
    println!();

    let mut matrix_3 = matrix_1.multiply(&matrix_2);
    matrix_3.display();
}

