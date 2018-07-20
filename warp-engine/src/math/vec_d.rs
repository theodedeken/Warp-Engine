//! Mathematical vector and handy methods.

use wasm_bindgen::prelude::*;

/// Vector of type `f64`.
#[wasm_bindgen]
pub struct VecD {
    values: Vec<f64>,
}

#[wasm_bindgen]
impl VecD {
    /// Constructs a vector with values.
    ///
    /// # Arguments
    /// * `values` - Array with the values of the vector.
    pub fn new(values: Vec<f64>) -> VecD {
        VecD { values }
    }

    /// Constructs an empty vector.
    ///
    /// # Arguments
    /// * `size` - The size of the vector.
    pub fn with_size(size: usize) -> VecD {
        let mut values = Vec::with_capacity(size);
        for _ in 0..size {
            values.push(0.0);
        }
        VecD { values }
    }

    /// Returns the length of the vector.
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns an element of the vector.
    ///
    /// # Arguments
    /// * `index` - Index to access.
    pub fn get(&self, index: usize) -> f64 {
        self.values[index]
    }

    /// Set an element of the vector.
    ///
    /// #Arguments
    /// * `index` - Index of element to change.
    /// * `value` - New value of the element.
    pub fn set(&mut self, index: usize, value: f64) {
        self.values[index] = value;
    }

    /// Performs an addition of two vectors and returns a new `VecD` containing the result.
    ///
    /// # Arguments
    /// * `other` - The vector to be added to this vector.        
    pub fn add(&self, other: &VecD) -> VecD {
        if (self.len() != other.len()) {
            panic!(
                "Vectors should have the same dimension! {} != {}",
                self.len(),
                other.len()
            );
        }
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            result.push(self.values[i] + other.get(i));
        }
        VecD::new(result)
    }

    /// Performs a subtraction of two vectors and returns a new `VecD` containing the result.
    ///
    /// # Arguments
    /// * `other` - The vector to be added to this vector.   
    pub fn sub(&self, other: &VecD) -> VecD {
        if (self.len() != other.len()) {
            panic!(
                "Vectors should have the same dimension! {} != {}",
                self.len(),
                other.len()
            );
        }
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            result.push(self.values[i] - other.get(i));
        }
        VecD::new(result)
    }

    /// Performs a multiplication with a value and returns a new `VecD` containing the result.
    ///
    /// # Arguments
    /// * `a` - Value to multiply with.  
    pub fn mult(&self, a: f64) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            result.push(self.values[i] * a);
        }
        VecD::new(result)
    }

    /// Performs a division with a value and returns a new `VecD` containing the result.
    ///
    /// # Arguments
    /// * `a` - Value to divide by.
    pub fn div(&self, a: f64) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(self.len());
        for i in 0..self.len() {
            result.push(self.values[i] / a);
        }
        VecD::new(result)
    }

    /// Returns the dot product of the two vectors.
    ///
    /// # Arguments
    /// * `other` - The second vector in the dot product.
    pub fn dot(&self, other: &VecD) -> f64 {
        if (self.len() != other.len()) {
            panic!(
                "Vectors should have the same dimension! {} != {}",
                self.len(),
                other.len()
            );
        }
        let mut result = 0.;
        for i in 0..self.len() {
            result += self.values[i] * other.get(i);
        }
        result
    }
}
