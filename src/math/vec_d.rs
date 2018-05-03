//! Mathematical vector of type `f64` and handy methods.

/// Vector of type `f64`.
pub struct VecD {
    values: Vec<f64>, 
}

impl VecD {
    /// Constructs a vector with elements of type `f64` with the given values.
    /// 
    /// # Arguments
    /// * `values` - Array with the values of the vector.
    #[no_mangle]
    pub extern fn new(values: Vec<f64>) -> VecD {
        VecD { values }
    }

    /// Constructs a vector with elements of type `f64` with the specified size.
    /// 
    /// # Arguments
    /// * `size` - The size of the vector.
    #[no_mangle]
    pub extern fn with_capacity(size: usize) -> VecD {
        VecD { values: Vec::with_capacity(size) }
    }

    /// Returns the length of the vector.
    #[no_mangle]
    pub extern fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns the element at the specified index.
    /// 
    /// # Arguments
    /// * `index` - Index to access.
    #[no_mangle]
    pub extern fn get(&self, index: usize) -> Option<&f64> {
        self.values.get(index)
    }

    /// Set the element at the specified index.
    /// 
    /// #Arguments
    /// * `index` - Index of element to change.
    /// * `value` - New value of the element.
    #[no_mangle]
    pub extern fn set(&mut self, index: usize, value: f64) {
        self.values[index] = value;
    }

    /// Performs an addition of two vectors and returns a new `VecD` containing the result.
    /// 
    /// # Arguments
    /// * `other` - The vector to be added to this vector.        
    #[no_mangle]
    pub extern fn plus(&self, other: &VecD) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            if let Some(b) = other.get(i) {
                result[i] = self.values[i] + b;
            }
        }
        VecD::new(result)
    }

    /// Performs a subtraction of two vectors and returns a new `VecD` containing the result.
    /// 
    /// # Arguments
    /// * `other` - The vector to be added to this vector.   
    #[no_mangle]
    pub extern fn minus(&self, other: &VecD) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            if let Some(b) = other.get(i) {
                result[i] = self.values[i] - b;
            }
        }
        VecD::new(result)
    }

    /// Performs a multiplication with a value and returns a new `VecD` containing the result.
    /// 
    /// # Arguments
    /// * `a` - Value to multiply with.   
    #[no_mangle]
    pub extern fn mult(&self, a: f64) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {            
            result[i] = self.values[i] * a;
        }
        VecD::new(result)
    }

    /// Performs a division with a value and returns a new `VecD` containing the result.
    /// 
    /// # Arguments
    /// * `a` - Value to divide by. 
    #[no_mangle]
    pub extern fn div(&self, a: f64) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {            
            result[i] = self.values[i] / a;
        }
        VecD::new(result)
    }

    /// Returns the dot product of the two vectors.
    /// 
    /// # Arguments
    /// * `other` - The second vector in the dot product. 
    #[no_mangle]
    pub extern fn dot(&self, other: &VecD) -> f64 {
        let mut result = 0.;
        for i in 0..self.len() {
            if let Some(b) = other.get(i) {
                result += self.values[i] * b;
            }
        }
        result
    }
}
