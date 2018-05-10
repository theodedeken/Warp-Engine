//! Row major matrix of type `f64` and handy methods.

use math::vec_d::VecD;
use wasm_bindgen::prelude::*;

/// Matrix of type `f64`.
#[wasm_bindgen]
pub struct MatD {
    width: usize,
    height: usize,
    values: Vec<f64>,
}

#[wasm_bindgen]
impl MatD {
    /// Construct a matrix with values.
    ///
    /// # Arguments
    /// * `width` - The width(columns) of the matrix.
    /// * `height` - The height(rows) of the matrix.
    /// * `values` - The elements of the matrix.
    pub fn new(width: usize, height: usize, values: Vec<f64>) -> MatD {
        MatD {
            width,
            height,
            values,
        }
    }

    /// Construct an empty matrix.
    ///
    /// # Arguments
    /// * `width` - The width(columns) of the matrix.
    /// * `height` - The height(rows) of the matrix.
    pub fn with_dimension(width: usize, height: usize) -> MatD {
        MatD {
            width,
            height,
            values: Vec::with_capacity(width * height),
        }
    }

    /// Get an element in the matrix.
    ///
    /// # Arguments
    /// * `row` - The row of the element.
    /// * `col` - The column of the element.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.values[row * self.width + col]
    }

    /// Set an element in the matrix.
    ///
    /// # Arguments
    /// * `row` - The row of the element.
    /// * `col` - The column of the element.
    /// * `value` - The new value for the element.
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.values[row * self.width + col] = value;
    }

    /// Return the width of the matrix.
    pub fn get_width(&self) -> usize {
        self.width
    }

    /// Return the height of the matrix.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Return a new matrix corresponding to the transposition of this matrix.
    pub fn transpose(&self) -> MatD {
        let mut values: Vec<f64> = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                values[i * self.height + j] = self.values[i + j * self.width]
            }
        }
        MatD::new(self.height, self.width, values)
    }

    /// Multiply this matrix with a vector and return a new vector with the result.
    ///
    /// # Arguments
    /// `vector` - The vector to multiply with.
    pub fn vec_mult(&self, vector: VecD) -> VecD {
        let mut result: Vec<f64> = Vec::with_capacity(vector.len());
        for i in 0..self.height {
            for j in 0..self.width {
                result[i] += self.values[i * self.width + j] * vector.get(j);
            }
        }
        VecD::new(result)
    }

    /// Multiply this matrix with another matrix and return a new matrix with the result.
    ///
    /// # Arguments
    /// `matrix` - The matrix to multiply with.
    pub fn mat_mult(&self, matrix: MatD) -> MatD {
        let width = matrix.get_width();
        let height = self.height;
        let mut result: Vec<f64> = Vec::with_capacity(width * height);
        for i in 0..height {
            for j in 0..width {
                for k in 0..self.width {
                    result[i * width + j] +=
                        self.values[i * self.width + k] * matrix.values[k * width + j];
                }
            }
        }
        MatD::new(width, height, result)
    }

    /// Add this matrix to another matrix and return a new matrix with the result.
    ///
    /// # Arguments
    /// `matrix` - The matrix to add.
    pub fn add(&self, matrix: MatD) -> MatD {
        let mut values: Vec<f64> = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                values[i * self.height + j] = self.values[i + j * self.width] + matrix.get(i, j)
            }
        }
        MatD::new(self.height, self.width, values)
    }

    /// Subtract another matrix width this matrix and return a new matrix with the result.
    ///
    /// # Arguments
    /// `matrix` - The matrix to subtract.
    pub fn sub(&self, matrix: MatD) -> MatD {
        let mut values: Vec<f64> = Vec::with_capacity(self.width * self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                values[i * self.height + j] = self.values[i + j * self.width] - matrix.get(i, j)
            }
        }
        MatD::new(self.height, self.width, values)
    }
}
