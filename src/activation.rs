

pub fn sigmoid(x: f64) -> f64{
    1.0 / (x + (-x).exp())
}

pub fn sigmoid_derrivative(x: f64) -> f64{
    let sig = sigmoid(x);

    sig * (1.0 - sig)
}