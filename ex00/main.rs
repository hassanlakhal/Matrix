use std::ops::{AddAssign, SubAssign, MulAssign};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

#[derive(Debug, Clone)]
struct Matrix<K> {
    data: Vec<Vec<K>>, // Column-major
    rows: usize,
    cols: usize,
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn size(&self) -> usize {
        self.rows * self.cols
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }
}


impl<K> Vector<K> 
where K: Copy + AddAssign + SubAssign + MulAssign 
{
    pub fn from(values: Vec<K>) -> Self {
        Self { data: values }
    }

    pub fn add(&mut self, v: &Vector<K>) {
        if self.size() != v.size() { return; }
        for i in 0..self.size() { self.data[i] += v.data[i]; }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        if self.size() != v.size() { return; }
        for i in 0..self.size() { self.data[i] -= v.data[i]; }
    }

    pub fn scl(&mut self, a: K) {
        for x in self.data.iter_mut() { *x *= a; }
    }
}

impl<K> Matrix<K> 
where K: Copy + AddAssign + SubAssign + MulAssign 
{
    pub fn from(values: Vec<Vec<K>>) -> Self {
        let cols = values.len();
        let rows = if cols > 0 { values[0].len() } else { 0 };
        Self { data: values, rows, cols }
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() { return; }
        for c in 0..self.cols {
            for r in 0..self.rows {
                self.data[c][r] += v.data[c][r];
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        if self.shape() != v.shape() { return; }
        for c in 0..self.cols {
            for r in 0..self.rows {
                self.data[c][r] -= v.data[c][r];
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for col in self.data.iter_mut() {
            for val in col.iter_mut() { *val *= a; }
        }
    }
}

fn main() {
    let v = Vector::from(vec![1.0, 2.0, 3.0]);
    println!("Vector dimension (size): {}", v.size()); // 3

    let m = Matrix::from(vec![
        vec![1.0, 3.0], // Column 1
        vec![2.0, 4.0], // Column 2
    ]);

    let (rows, cols) = m.shape();
    println!("Matrix Shape: {} rows x {} columns", rows, cols); // 2x2
    println!("Matrix total elements: {}", m.size()); // 4
    println!("Is matrix square? {}", m.is_square()); // true
}