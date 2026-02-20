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
    fn dot (&self, v: &Vector<K>) -> K
    where
        K: Add<Output = K> + Mul<Output = K> + Default
    {
        let mut res = K::default();
        if self.data.len() != v.data.len() {return res}
        for i in 0..self.data.len() {
            res = self.data[i] * v.data[i] + res;
        }
        res
    }
}

impl<K: KField> Matrix<K> {
    fn mul_vec(&self, vec: &Vector<K>) -> Vector<K> {
        let mut result = Vector { data: vec![K::default(); self.rows] };

        for r in 0..self.rows {
            let mut row_vec = Vector { data: Vec::with_capacity(self.cols) };

            for c in 0..self.cols {
                row_vec.data.push(self.data[c * self.rows + r]);
            }

            result.data[r] = row_vec.dot(vec);
        }

        result
    }
    fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K>{

        
    }
}

fn main() {
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [2.]
    let u = Matrix::from([
    [2., 0.],
    [0., 2.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [8.]
    // [4.]
    let u = Matrix::from([
    [2., -2.],
    [-2., 2.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(&v));
    // [4.]
    // [-4.]
    // let u = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // ]);
    // let v = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // ]);
    // println!("{}", u.mul_mat(&v));
    // // [1., 0.]
    // // [0., 1.]
    // let u = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // ]);
    // let v = Matrix::from([
    // [2., 1.],
    // [4., 2.],
    // ]);
    // println!("{}", u.mul_mat(&v));
    // // [2., 1.]
    // // [4., 2.]
    // let u = Matrix::from([
    // [3., -5.],
    // [6., 8.],
    // ]);
    // let v = Matrix::from([
    // [2., 1.],
    // [4., 2.],
    // ]);
    // println!("{}", u.mul_mat(&v));
    // // [-14., -7.]
    // // [44., 22.]
}