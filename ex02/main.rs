use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

#[derive(Debug, Clone)]
struct Matrix<K> {
    data: Vec<Vec<K>>,
    rows: usize,
    cols: usize,
}

trait Scalar: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Copy + Debug {}
impl<T> Scalar for T where T: Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy + Debug {}

impl<K: Scalar> Matrix<K> {
    fn from(data: Vec<Vec<K>>) -> Self {
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Matrix { data, rows, cols }
    }

    fn add(&mut self, v: &Matrix<K>) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = self.data[i][j] + v.data[i][j];
            }
        }
    }

    fn sub(&mut self, v: &Matrix<K>) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = self.data[i][j] - v.data[i][j];
            }
        }
    }

    fn scl(&mut self, a: K) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = self.data[i][j] * a;
            }
        }
    }
}

impl<K: Scalar> Vector<K> {
    fn from(data: Vec<K>) -> Self {
        Vector { data }
    }

    fn add(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] + v.data[i];
        }
    }

    fn sub(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] - v.data[i];
        }
    }

    fn scl(&mut self, a: K) {
        for i in 0..self.data.len() {
            self.data[i] = self.data[i] * a;
        }
    }
}

trait Lerpable<T> {
    fn lerp(u: &Self, v: &Self, t: T) -> Self;
}

impl Lerpable<f32> for f32 {
    fn lerp(u: &f32, v: &f32, t: f32) -> f32 {
        *u + (*v - *u) * t
    }
}

impl<K: Scalar> Lerpable<K> for Vector<K> {
    fn lerp(u: &Vector<K>, v: &Vector<K>, t: K) -> Vector<K> {
        let mut res = v.clone();
        res.sub(u);
        res.scl(t);
        res.add(u);
        res
    }
}

impl<K: Scalar> Lerpable<K> for Matrix<K> {
    fn lerp(u: &Matrix<K>, v: &Matrix<K>, t: K) -> Matrix<K> {
        let mut res = v.clone();
        res.sub(u);
        res.scl(t);
        res.add(u);
        res
    }
}

fn lerp<V, T>(u: V, v: V, t: T) -> V 
where V: Lerpable<T> 
{
    V::lerp(&u, &v, t)
}

fn main() {
    println!("Float lerp: {:?}", lerp(100f32, 450f32, 0.5f32));

    let v1 = Vector::from(vec![1, 2]);
    let v2 = Vector::from(vec![3, 4]);
    let vmid = lerp(v1, v2, 1);
    println!("Vector lerp: {:?}", vmid);

    let m1 = Matrix::from(vec![vec![2.0, 1.0], vec![3.0, 4.0]]);
    let m2 = Matrix::from(vec![vec![20.0, 10.0], vec![30.0, 40.0]]);
    let mid = lerp(m1, m2, 0.5f32);
    println!("Matrix lerp: {:?}", mid);
}