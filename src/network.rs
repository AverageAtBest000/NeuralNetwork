use crate::network;

use crate::layer::Layer;
use crate::matrix::Matrix;


struct Network{
    layers: Vec<Layer> 
}

impl Network{
    
    pub fn new(layers: Vec<Layer>) -> Network{
        Network{layers}
    }

    pub fn forward_propagate(self, input_layer : Matrix) -> Matrix{
        
        let layer_out = self.layer[0].forward_propagate(&input_layer);

        for layer in 1..self.layers.len(){
            layer_out = self.layers[i].forward_propagate(&layer_out);
        }

        layer_out 
    } 

}









