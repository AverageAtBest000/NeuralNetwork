struct Matrix{
    cols: usize,
    rows: usize,
    elements: Vec<f64>
}


impl Matrix{
    
    fn new(cols: usize, rows: usize) -> Matrix{
        let matrix = Matrix{
            cols: cols,
            rows: rows,
            Vec::new()
        }

        matrix
    }



}