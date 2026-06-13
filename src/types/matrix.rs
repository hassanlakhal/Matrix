use super::field::Field;
use std::fmt;
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
    // pub fn sub(&mut self, v: Matrix<K>){

    // }
    // pub fn scl(&mut self, a: K){

    // }
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