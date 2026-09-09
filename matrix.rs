struct Matrix{
    cols: usize,
    rows: usize,
    elements: Vec<f64>
}


impl Matrix{

    fn new( rows: usize, cols: usize) -> Matrix{
        let matrix = Matrix{
            cols,
            rows,
            elements: vec![0.0; cols*rows],
        };
        matrix
    }

    fn from_vector(rows:usize, cols:usize, elements:Vec<f64> ) -> Matrix{
        assert_eq(rows * cols, elements.len());
        
        let matrix = Matrix{
            rows,
            cols,
            elements,
        };
        matrix
    }

    fn get( &self, row: usize, col: usize) -> f64{
        let element = self.elements[ row * self.cols + col];
        element
    }

    
    fn add( &self, matrix2: &Self  ) -> Matrix{

        assert_eq!(self.rows, matrix2.rows);
        assert_eq!(self.cols, matrix2.cols);

        let mut newElements = vec![0.0; self.elements.len()];
        
        for i in 0..self.elements.len(){
            newElements[i] = self[i] + matrix2[i]
        }

        let newMaxtix = Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: newElements,
        };
        
        newMaxtix

    }




}