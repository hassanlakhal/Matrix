use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait KField: 
    Copy + Default + PartialEq + fmt::Display +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign 
{}

impl KField for f32 {}

#[derive(Debug, Clone)]
pub struct Vector<K> { pub data: Vec<K> }

#[derive(Debug, Clone)]
pub struct Matrix<K> {
    pub data: Vec<K>, 
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


pub trait Lerp {
    fn lerp(u: Self, v: Self, t: f32) -> Self;
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    V::lerp(u, v, t)
}

impl Lerp for f32 {
    fn lerp(u: f32, v: f32, t: f32) -> f32 {
        u + (v - u) * t
    }
}

impl Lerp for Vector<f32> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        let mut res = v.clone();
        res.sub(&u);
        res.scl(t);
        res.add(&u);
        res
    }
}

impl Lerp for Matrix<f32> {
    fn lerp(u: Self, v: Self, t: f32) -> Self {
        let mut res = v.clone();
        res.sub(&u);
        res.scl(t);
        res.add(&u);
        res
    }
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for val in &self.data { writeln!(f, "[{:.1}]", val)?; }
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

fn main() {
    // Test Scalars
    println!("{}", lerp(0., 1., 0.));
    println!("{}", lerp(0., 1., 1.));
    println!("{}", lerp(0., 1., 0.5));
    println!("{:.1}", lerp(21., 42., 0.3));

    // Test Vector
    println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));

    // Test Matrix
    println!("{}", lerp(
        Matrix::from([[2., 1.], [3., 4.]]), 
        Matrix::from([[20., 10.], [30., 40.]]), 
        0.5
    ));
}