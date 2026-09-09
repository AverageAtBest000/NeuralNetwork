struct Matrix{
    cols: usize,
    rows: usize,
    elements: Vec<f64>
}


impl Matrix{

    fn new( rows: usize, cols: usize) -> Matrix{
        let matrix = Matrix{
            cols: cols,
            rows: rows,
            elements: vec![0.0; cols*rows],
        }

        matrix
    }

    





}