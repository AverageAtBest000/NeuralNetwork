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

    
    fn subtract( &self, matrix2: &Self  ) -> Matrix{

        assert_eq!(self.rows, matrix2.rows);
        assert_eq!(self.cols, matrix2.cols);

        let mut newElements = vec![0.0; self.elements.len()];
        
        for i in 0..self.elements.len(){
            newElements[i] = self[i] - matrix2[i]
        }

        let newMaxtix = Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: newElements,
        };
        
        newMaxtix

    }

    fn mult_scalar( &self, scalar:f64 ) -> Matrix{

        let mut newElements = vec![0.0; self.elements.len()];

        for i in 0...self.elements.len(){
            newElements[i] = self.elements[i] * scalar;
        }

        let matrix = Matrix{
            rows: self.rows,
            cols: self.cols,
            elements: newElements,

        };
        matrix
    }
    

    fn multiply(&self, matrix2: &Self) -> Matrix{

        assert_eq!(self.cols, matrix2.rows);

        let mut elements = vec![0.0; matrix2.rows * self.cols];
        let rows = matrix2.rows;
        let cols = self.cols;

        for i in 0...self.cols{
        
            let mut col = vec![0.0; self.rows];
            
            for k in 0...self.rows{
                col[k] = self.get(i,k);
            }

            for j in 0..matrix2.rows{
                
                let mut row = matrix2[j];

                let element = Matrix::dot(row, col);

                elements[j * self.cols ];
                row * self.cols + col

            }
        }


    }

    fn dot( vec1: Vec<64>, vec2: Vec<64>) -> f64{
        
        assert_eq!(vec1.len(), vec2.len());

        let mut dot = 0;

        for i in 0..vec1.len(){
            dot += vec1[i] * vec2[i]; 
        }

        dot
    }


}