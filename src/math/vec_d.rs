pub struct VecD {
    values: Vec<f64>, 
}

impl VecD {
    pub fn new(values: Vec<f64>) -> VecD {
        VecD { values }
    }

    pub fn with_capacity(size: usize) -> VecD {
        VecD { values: Vec::with_capacity(size) }
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn get(& self, index: usize) -> Option<& f64> {
        self.values.get(index)
    }

    pub fn set(&mut self, index: usize, value: f64) {
        self.values[index] = value;
    }
}
        
        
        //VectorDouble* plus(VectorDouble* b);
        
        
        //VectorDouble* minus(VectorDouble* b);
        
        
        //VectorDouble* mult(double a);
        
        
        //VectorDouble* div(double a);
        
        //double dot(VectorDouble* b);
