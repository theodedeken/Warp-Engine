//! Row major matrix of type `f64` and handy methods.

use math::vec_d::VecD;
use wasm_bindgen::prelude::*;

/// Matrix of type `f64`.
#[wasm_bindgen]
pub struct MatD {
    rows: usize,
    cols: usize,
    values: Vec<f64>,
}

/// Methods that get binded to javascript
#[wasm_bindgen]
impl MatD {
    /// Construct a matrix with values.
    ///
    /// # Arguments
    /// * `rows` - The number of rows in the matrix.
    /// * `cols` - The number of columns in the matrix.
    /// * `values` - The elements of the matrix.
    pub fn new(rows: usize, cols: usize, values: Vec<f64>) -> MatD {
        if rows * cols != values.len() {
            panic!(
                "provided an incorrect amount of values: got {} values but matrix has dimensions {}x{}", 
                values.len(), 
                rows, 
                cols
            );
        }
        MatD {
            rows,
            cols,
            values,
        }
    }

    /// Construct an empty matrix.
    ///
    /// # Arguments
    /// * `rows` - The number of rows in the matrix.
    /// * `cols` - The number of columns in the matrix.
    pub fn with_dimension(rows: usize, cols: usize) -> MatD {
        let mut values = Vec::with_capacity(rows * cols);
        for _ in 0..rows * cols {
            values.push(0.0);
        }
        MatD {
            rows,
            cols,
            values,
        }
    }

    /// Construct a matrix from a vector
    /// 
    /// # Arguments
    /// * `rows` - The number of rows in the matrix
    /// * `cols` - The number of columns in the matrix
    /// * `vector` - The vector to convert to a matrix
    pub fn from_vec(rows: usize, cols: usize, vector: VecD) -> MatD {
        if rows * cols != vector.len() {
            panic!(
                "Vector does not fit in provided dimensions: got vector with len {} but matrix has dimensions {}x{}", 
                vector.len(), 
                rows, 
                cols
            );
        }
        MatD {rows, cols, values: vector.values() }
    }

    /// Get an element in the matrix.
    ///
    /// # Arguments
    /// * `row` - The row of the element.
    /// * `col` - The column of the element.
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.values[row * self.cols + col]
    }

    /// Set an element in the matrix.
    ///
    /// # Arguments
    /// * `row` - The row of the element.
    /// * `col` - The column of the element.
    /// * `value` - The new value for the element.
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.values[row * self.cols + col] = value;
    }

    /// Return the rows of the matrix.
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Return the cols of the matrix.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Return a new matrix corresponding to the transposition of this matrix.
    pub fn transpose(&self) -> MatD {
        let mut values: Vec<f64> = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.cols {
            for j in 0..self.rows {
                values.push(self.values[i + j * self.cols]);
            }
        }
        MatD::new(self.cols, self.rows, values)
    }

    /// Multiply this matrix with a vector and return a new vector with the result.
    ///
    /// # Arguments
    /// `vector` - The vector to multiply with.
    pub fn vec_mult(&self, vector: VecD) -> VecD {
        if self.rows != vector.len() {
            panic!("Matrix and vector should have the same dimension: {} != {}", self.rows, vector.len());
        }
        let mut result = VecD::with_size(vector.len());
        for i in 0..self.rows {
            let mut acc = 0.0;
            for j in 0..self.cols {
                acc += self.values[i * self.rows + j] * vector.get(j);
            }
            result.set(i, acc);
        }
        result
    }

    /// Multiply this matrix with another matrix and return a new matrix with the result.
    ///
    /// # Arguments
    /// `matrix` - The matrix to multiply with.
    pub fn mat_mult(&self, matrix: MatD) -> MatD {
        if self.cols != matrix.rows() {
            panic!("Matrices dont have corresponding dimensions: {} != {}", self.cols, matrix.rows());
        }
        let rows = matrix.cols();
        let cols = self.rows;
        let mut result: Vec<f64> = Vec::with_capacity(rows * cols);
        for i in 0..rows {
            for j in 0..cols {
                result.push(0.0);
                for k in 0..self.cols {
                    result[i * cols + j] +=
                        self.values[i * self.cols + k] * matrix.values[k * rows + j];
                }
            }
        }
        MatD::new(rows, cols, result)
    }

    /// Add this matrix to another matrix and return a new matrix with the result.
    ///
    /// # Arguments
    /// `matrix` - The matrix to add.
    pub fn add(&self, matrix: MatD) -> MatD {
        if  self.rows != matrix.rows() ||self.cols != matrix.cols() {
            panic!("Dimensions of matrices dont correspond: {}x{} != {}x{}", self.rows, self.cols, matrix.rows(), matrix.cols());
        }
        let mut values: Vec<f64> = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                values.push(self.values[i * self.cols + j] + matrix.get(i, j))
            }
        }
        MatD::new(self.rows, self.cols, values)
    }

    /// Subtract another matrix to this matrix and return a new matrix with the result.
    ///
    /// # Arguments
    /// `matrix` - The matrix to subtract.
    pub fn sub(&self, matrix: MatD) -> MatD {
        if  self.rows != matrix.rows() ||self.cols != matrix.cols() {
            panic!("Dimensions of matrices dont correspond: {}x{} != {}x{}", self.rows, self.cols, matrix.rows(), matrix.cols());
        }
        let mut values: Vec<f64> = Vec::with_capacity(self.rows * self.cols);
        for i in 0..self.rows {
            for j in 0..self.cols {
                values.push(self.values[i * self.cols + j] - matrix.get(i, j));
            }
        }
        MatD::new(self.rows, self.cols, values)
    }
}

/// Methods that don't get binded to javascript
impl MatD {
    /// Return the value array of the matrix.
    pub fn values(self)-> Vec<f64> {
        self.values
    }
}