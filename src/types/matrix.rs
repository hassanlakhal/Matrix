use super::field::Field;
use std::fmt;
#[derive(Debug, Clone, PartialEq)]

pub struct Matrix<K: Field>{
    pub data: Vec<Vec<K>>,
    pub rows: usize,
    pub cols: usize
}

impl<K: Field> Matrix<K>{
    pub fn form(data: Vec<Vec<K>>) -> Self{
        let rows = data.len();
        let cols = if rows == 0 {0} else {data[0].len()};

        assert!(
            data.iter().all(|r| r.len() == cols),
            "All rows must have the same number of columns"
        );

        Self{ data ,rows , cols}
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