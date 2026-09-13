use crate::matrix;

pub struct Matrix{
    cols: usize,
    rows: usize,
    elements: Vec<f64>
}


impl Matrix{

    pub fn new( rows: usize, cols: usize) -> Matrix{
        Matrix{
            cols,
            rows,
            elements: vec![0.0; cols*rows],
        }
        
    }

    pub fn from_vector(rows:usize, cols:usize, elements:Vec<f64> ) -> Matrix{
        assert_eq!(rows * cols, elements.len());

        Matrix{
            rows,
            cols,
            elements,
        }
    }

    pub fn get( &self, row: usize, col: usize) -> f64{
        let element = self.elements[ row * self.cols + col];
        element
    }

    
    pub fn add( &self, matrix2: &Self  ) -> Matrix{

        assert_eq!(self.rows, matrix2.rows);
        assert_eq!(self.cols, matrix2.cols);

        let mut new_elements = vec![0.0; self.elements.len()];
        
        for i in 0..self.elements.len(){
            new_elements[i] = self.elements[i] + matrix2.elements[i]
        }

        Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: new_elements,
        }
    }

    
    pub fn subtract( &self, matrix2: &Self  ) -> Matrix{

        assert_eq!(self.rows, matrix2.rows);
        assert_eq!(self.cols, matrix2.cols);

        let mut new_elements = vec![0.0; self.elements.len()];
        
        for i in 0..self.elements.len(){
            new_elements[i] = self.elements[i] - matrix2.elements[i]
        }

        Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: new_elements,
        }
        
    }

    pub fn mult_scalar( &self, scalar:f64 ) -> Matrix{

        let mut new_elements = vec![0.0; self.elements.len()];

        for i in 0..self.elements.len(){
            new_elements[i] = self.elements[i] * scalar;
        }

        Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: new_elements,

        }
    }
    

    pub fn multiply(&self, matrix2: &Self) -> Matrix{

        assert_eq!(self.cols, matrix2.rows);

        let mut elements = vec![0.0; self.rows * matrix2.cols];
        let rows = self.rows;
        let cols = matrix2.cols;

        for i in 0..matrix2.cols{
        
            let mut col = &mut vec![0.0; matrix2.rows];
            
            for k in 0..matrix2.rows{
                col[k] = matrix2.get(k,i);
            }

            for j in 0..self.rows{
                
                let mut row = &self.elements[ j*self.cols..( j+1 )*self.cols ];

                let element = Matrix::dot(row, col);

                elements[j * cols + i ] = element;
                // (self.row, m2.col)
            }
        }

        Matrix {
            rows,
            cols,
            elements,
        }

    }

    pub fn dot( vec1: &[f64], vec2: &[f64]) -> f64{
        
        assert_eq!(vec1.len(), vec2.len());

        let mut dot: f64 = 0.0;

        for i in 0..vec1.len(){
            dot += vec1[i] * vec2[i]; 
        }

        dot
    }


    pub fn display(&self){
        for i in (0..self.elements.len()).step_by(self.cols){
            let row = &self.elements[i..(i+self.cols)];
            println!("{:?}",row);
        }
    }

    pub fn transpose(matrix: &Self) -> Matrix{

        let mut new_elements = vec![0.0; matrix.rows * matrix.cols];

        for r in 0..matrix.rows{
            for c in 0..matrix.cols{
                new_elements[c * matrix.rows + r ] = matrix.get(r,c);
            }

        }

        Matrix{
            rows: matrix.cols,
            cols: matrix.rows,
            elements: new_elements,
        }
    }


    pub fn map<F>(&self, operation: F) -> Matrix
    where
    F: Fn(f64) -> f64, 
    {
        let mut new_elements = vec![0.0; self.rows * self.cols];

        for i in 0..self.elements.len(){
            new_elements[i] = operation(self.elements[i]);
        }

        Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: new_elements,
        }

    }

}