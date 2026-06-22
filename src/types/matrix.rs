use super::field::Field;
use std::fmt;
use crate::Vector;

#[derive(Debug, Clone, PartialEq)]

pub struct Matrix<K: Field>{
    pub data: Vec<Vec<K>>,
    pub rows: usize,
    pub cols: usize
}

impl<K: Field> Matrix<K>{

    pub fn from<I, V>(data: I) -> Self 
    where
        I: Into<Vec<V>>, 
        V: Into<Vec<K>>,
    {
        let converted: Vec<Vec<K>> = data
            .into()                     
            .into_iter()                 
            .map(|row| row.into())        
            .collect();                   
        let rows = converted.len();
        let cols = if rows == 0 {0} else {converted[0].len()};
        Matrix { data: converted , rows, cols}
    
    }

    pub fn shape(&self) -> (usize, usize){
        (self.rows, self.cols)
    }

    pub fn is_square(&self) -> bool{
        self.rows == self.cols
    }

    pub fn get(&self, row: usize, col: usize) -> K{
        self.data[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, val: K){
        self.data[row][col] = val
    }

    pub fn assert_same_shape(&self, other: &Self) {
        assert_eq!(
            self.shape(), other.shape(),
            "Matrix shape mismatch: {:?} vs {:?}",
            self.shape(), other.shape()
        );
    }

    pub fn add(&mut self, v: Matrix<K>){
        
        for i in 0..self.rows {
            for j in 0..self.cols{
                self.set(i,j, self.data[i][j] + v.data[i][j]);
            }
        }
    }
    pub fn sub(&mut self, v: Matrix<K>){
        for i in 0..self.rows {
            for j in 0..self.cols{
                self.set(i,j, self.data[i][j] - v.data[i][j]);
            }
        }
    }
    pub fn scl(&mut self, a: K){
        for i in 0..self.rows {
            for j in 0..self.cols{
                self.set(i,j, self.data[i][j] * a);
            }
        }
    }
    pub fn mul_vec(&self, vec: Vector<K>) -> Vector<K> {
        
        let mut result_data = vec![K::zero(); self.rows];       
        
        for i in 0..self.rows {
            let mut sum = K::zero();
            for j in 0..self.cols { 
                sum += self.data[i][j].clone() * vec.data[j].clone(); 
            }
            result_data[i] = sum;
        }
        
        Vector::from(result_data)
    }

    pub fn mul_mat(&self, other: Matrix<K>) -> Matrix<K> {
        assert_eq!(self.cols, other.rows, "Error: The number of columns in A must equal the number of rows in B!");
        let mut result_data = vec![vec![K::zero(); other.cols]; self.rows];

        for i in 0..self.rows {         
            for j in 0..other.cols {   
                let mut sum = K::zero();
                
                for k in 0..self.cols { 
                    sum += self.data[i][k].clone() * other.data[k][j].clone();
                }

                result_data[i][j] = sum;
            }
        }

        Matrix::from(result_data)
    }

    pub fn trace(&self) -> K{

        let mut sum = K::zero();
        for i in 0..self.rows {
            sum += self.data[i][i];
        }
        
        sum
    }

    pub fn transpose(&self) -> Matrix<K>{
        let mut result_data = vec![vec![K::zero(); self.rows]; self.cols];

        for i in 0..self.rows{
            for j in 0..self.cols{
                result_data[j][i] = self.data[i][j].clone();
            }
        }
        Matrix::from(result_data)
    }

    pub fn row_echelon(&mut self) -> Matrix<K> {
        let mut current_row = 0;

        for i in 0..self.cols {
            if current_row >= self.rows {
                break;
            }

            let mut max_row = current_row;
            for row in current_row + 1..self.rows {
                if self.data[row][i].abs() > self.data[max_row][i].abs() {
                    max_row = row;
                }
            }

            if self.data[max_row][i] == K::zero() {
                continue;
            }

            if current_row != max_row {
                for j in 0..self.data[current_row].len() {
                    let temp = self.data[current_row][j];
                    self.data[current_row][j] = self.data[max_row][j];
                    self.data[max_row][j] = temp;
                }
            }

            let pivot = self.data[current_row][i];
            for j in i..self.data[current_row].len() {
                self.data[current_row][j] = self.data[current_row][j] / pivot;
            }

            for row in current_row + 1..self.rows {
                let pivot = self.data[current_row][i];
                let factor = self.data[row][i] / pivot;
                for j in i..self.data[row].len() {
                        let current_row_value = self.data[current_row][j];
                        self.data[row][j] -= factor * current_row_value;
                }
            }

            current_row += 1;
        }

        Matrix::from(self.data.clone())
    }

    pub fn determinant(&mut self) -> K {
        if self.rows != self.cols {
            panic!("Determinant only defined for square matrices");
        }

        let mut swap_count = 0;
        let mut current_row = 0;
        let n = self.rows;

        for i in 0..self.cols {
            if current_row >= self.rows {
                break;
            }

            let mut max_row = current_row;
            for row in current_row + 1..self.rows {
                if self.data[row][i].abs() > self.data[max_row][i].abs() {
                    max_row = row;
                }
            }

            if self.data[max_row][i] == K::zero() {
                return K::zero();
            }

            if current_row != max_row {
                for j in 0..self.data[current_row].len() {
                    let temp = self.data[current_row][j];
                    self.data[current_row][j] = self.data[max_row][j];
                    self.data[max_row][j] = temp;
                }
                swap_count += 1;
            }

            for row in current_row + 1..self.rows {
                let pivot = self.data[current_row][i];
                let factor = self.data[row][i] / pivot;
                for j in i..self.data[row].len() {
                        let current_row_value = self.data[current_row][j];
                        self.data[row][j] -= factor * current_row_value;
                }
            }

            current_row += 1;
        }

        let mut det = K::one();
        for i in 0..n {
            det = det * self.data[i][i];
        }

        if swap_count % 2 == 1 {
            det = -det;
        }

        det
    }

    pub fn inverse(&mut self) -> Result<Matrix<K>, String> {
        if self.rows != self.cols {
            return Err("Matrix must be square".to_string());
        }

        let n = self.rows;

        let mut augmented = Vec::new();
        for i in 0..n {
            let mut row = self.data[i].clone();
            
            for j in 0..n {
                if i == j {
                    row.push(K::one());
                } else {
                    row.push(K::zero());
                }
            }
            augmented.push(row);
        }
        self.data = augmented;

        for i in 0..n {
            let mut max_row = i;
            for row in i + 1..n {
                if self.data[row][i].abs() > self.data[max_row][i].abs() {
                    max_row = row;
                }
            }

            if self.data[max_row][i] == K::zero() {
                return Err("Matrix is singular".to_string());
            }

            if i != max_row {
                for j in 0..2 * n {
                    let temp = self.data[i][j];
                    self.data[i][j] = self.data[max_row][j];
                    self.data[max_row][j] = temp;
                }
            }

            for row in i + 1..n {
                let pivot = self.data[i][i];
                let factor = self.data[row][i] / pivot;
                
                for j in i..2 * n {
                    let pivot_value = self.data[i][j];
                    self.data[row][j] -= factor * pivot_value;
                }
            }
        }

        for i in (0..n).rev() {
            let pivot = self.data[i][i];
            for j in 0..2 * n {
                self.data[i][j] = self.data[i][j] / pivot;
            }

            for row in 0..i {
                let pivot_val = self.data[row][i];
                for j in 0..2 * n {
                    let pivot_value = self.data[i][j];
                    self.data[row][j] -= pivot_val * pivot_value;
                }
            }
        }

        let mut inverse_data = Vec::new();
        for i in 0..n {
            let mut inv_row = Vec::new();
            for j in n..2 * n {
                inv_row.push(self.data[i][j]);
            }
            inverse_data.push(inv_row);
        }

        Ok(Matrix::from(inverse_data))
    }
}

impl<K: Field> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            let row_str: Vec<String> = row.iter().map(|v| format!("{}", v)).collect();
            writeln!(f, "[{}]", row_str.join(", "))?;
        }
        Ok(())
    }
}