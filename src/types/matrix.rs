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
        assert_eq!(self.cols, other.rows, "Error: Columns of A not equale Rows dyal of B!");

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