use crate::network;

use crate::layer::Layer;
use crate::matrix::Matrix;


 pub struct Network{
    layers: Vec<Layer> 
}

impl Network{
    
    pub fn new(layers: Vec<Layer>) -> Network{
        Network{layers}
    }

    pub fn forward_propagate(&self, input_layer : Matrix) -> Matrix{
        
        let mut layer_out = self.layers[0].forward(&input_layer);

        for layer in 1..self.layers.len(){
            layer_out = self.layers[layer].forward(&layer_out);
        }

        layer_out 
    } 

}









