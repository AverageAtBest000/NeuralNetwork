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
        let matrix = Matrix{
            rows,
            cols,
            elements,
        };
        matrix
    }

    fn get( row: usize, col: usize, &self) -> f64{
        let element = self.elements[ row*self.cols + col];
        element
    }

    





}