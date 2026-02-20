use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait KField: 
    Copy + Default + PartialEq + fmt::Display +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign 
{}

impl KField for f32 {}

pub struct Vector<K> { pub data: Vec<K> }

pub struct Matrix<K> {
    pub data: Vec<K>, // Column-Major
    pub rows: usize,
    pub cols: usize,
}

impl<K: KField> Vector<K> {
    pub fn from<const N: usize>(arr: [K; N]) -> Self {
        Self { data: arr.to_vec() }
    }
}

impl<K: KField> Matrix<K> {
    pub fn from<const R: usize, const C: usize>(arr: [[K; C]; R]) -> Self {
        let mut data = Vec::with_capacity(R * C);
        for c in 0..C {
            for r in 0..R {
                data.push(arr[r][c]);
            }
        }
        Self { data, rows: R, cols: C }
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for val in &self.data {
            writeln!(f, "[{:.1}]", val)?;
        }
        Ok(())
    }
}

impl<K: fmt::Display + Copy> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            write!(f, "[")?;
            for c in 0..self.cols {
                write!(f, "{:.1}{}", self.data[c * self.rows + r], if c == self.cols - 1 { "" } else { ", " })?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<K: KField> Vector<K> {
    pub fn add(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() { self.data[i] += v.data[i]; }
    }
    pub fn sub(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() { self.data[i] -= v.data[i]; }
    }
    pub fn scl(&mut self, a: K) {
        for val in &mut self.data { *val *= a; }
    }
}

impl<K: KField> Matrix<K> {
    pub fn add(&mut self, v: &Matrix<K>) {
        for i in 0..self.data.len() { self.data[i] += v.data[i]; }
    }
    pub fn sub(&mut self, v: &Matrix<K>) {
        for i in 0..self.data.len() { self.data[i] -= v.data[i]; }
    }
    pub fn scl(&mut self, a: K) {
        for val in &mut self.data { *val *= a; }
    }
}

fn main() {
    // Vector Example
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    u.add(&v);
    println!("Vector Add:\n{}", u); 

    // Matrix Example
    let mut mat_u = Matrix::from([
        [1., 2.],
        [3., 4.],
    ]);
    let mat_v = Matrix::from([
        [7., 4.],
        [-2., 2.],
    ]);
    mat_u.add(&mat_v);
    println!("Matrix Add:\n{}", mat_u);
}