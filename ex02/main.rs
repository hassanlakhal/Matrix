use std::fmt;
use std::ops::{AddAssign, SubAssign, MulAssign};


#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

#[derive(Debug, Clone)]
struct Matrix<K> {
    data: Vec<Vec<K>>, // Column-major storage
    rows: usize,
    cols: usize,
}


impl<K: Copy + AddAssign + SubAssign + MulAssign> Vector<K> {
    fn from<const N: usize>(arr: [K; N]) -> Self {
        Self { data: arr.to_vec() }
    }

    fn add(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() { self.data[i] += v.data[i]; }
    }
    fn sub(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() { self.data[i] -= v.data[i]; }
    }
    fn scl(&mut self, a: K) {
        for x in self.data.iter_mut() { *x *= a; }
    }
}

impl<K: Copy + AddAssign + SubAssign + MulAssign> Matrix<K> {
    fn from<const R: usize, const C: usize>(arr: [[K; C]; R]) -> Self {
        let mut cols_data = vec![vec![arr[0][0]; R]; C];
        for r in 0..R {
            for c in 0..C { cols_data[c][r] = arr[r][c]; }
        }
        Matrix { data: cols_data, rows: R, cols: C }
    }

    fn add(&mut self, v: &Matrix<K>) {
        for c in 0..self.cols {
            for r in 0..self.rows { self.data[c][r] += v.data[c][r]; }
        }
    }
    fn sub(&mut self, v: &Matrix<K>) {
        for c in 0..self.cols {
            for r in 0..self.rows { self.data[c][r] -= v.data[c][r]; }
        }
    }
    fn scl(&mut self, a: K) {
        for col in self.data.iter_mut() {
            for val in col.iter_mut() { *val *= a; }
        }
    }
}


trait Lerpable<K> {
    fn lerp(u: Self, v: Self, t: K) -> Self;
}

fn lerp<T, K>(u: T, v: T, t: K) -> T 
where T: Lerpable<K> 
{
    T::lerp(u, v, t)
}

impl Lerpable<f32> for f32 {
    fn lerp(u: f32, v: f32, t: f32) -> f32 {
        u + (v - u) * t
    }
}

impl<K: Copy + AddAssign + SubAssign + MulAssign> Lerpable<K> for Vector<K> {
    fn lerp(u: Self, v: Self, t: K) -> Self {
        let mut res = v.clone();
        res.sub(&u);
        res.scl(t);
        res.add(&u);
        res
    }
}

impl<K: Copy + AddAssign + SubAssign + MulAssign> Lerpable<K> for Matrix<K> {
    fn lerp(u: Self, v: Self, t: K) -> Self {
        let mut res = v.clone();
        res.sub(&u);
        res.scl(t);
        res.add(&u);
        res
    }
}


impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, val) in self.data.iter().enumerate() {
            write!(f, "[{}]", val)?;
            if i < self.data.len() - 1 { writeln!(f)?; }
        }
        Ok(())
    }
}

impl<K: fmt::Display + Copy> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for r in 0..self.rows {
            if r > 0 { write!(f, " ")?; }
            write!(f, "[")?;
            for c in 0..self.cols {
                write!(f, "{}", self.data[c][r])?;
                if c < self.cols - 1 { write!(f, ", ")?; }
            }
            write!(f, "]")?;
            if r < self.rows - 1 { writeln!(f)?; }
        }
        write!(f, "]")
    }
}


fn main() {
    println!("{}", lerp(0., 1., 0.));
    println!("{}", lerp(0., 1., 1.));
    println!("{}", lerp(0., 1., 0.5));
    println!("{:.1}", lerp(21., 42., 0.3));

    println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));

    println!("{}", lerp(
        Matrix::from([[2., 1.], [3., 4.]]), 
        Matrix::from([[20., 10.], [30., 40.]]), 
        0.5
    ));
}