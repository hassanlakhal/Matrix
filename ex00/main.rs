use std::{
    ops::{Add, Sub, Mul},
    fmt::Debug,
};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

#[derive(Debug, Clone)]
struct Matrix<K> {
    data: Vec<Vec<K>>,
    _rows: usize,
    _cols: usize,
}

trait Scalable<S> {
    fn scl(&mut self, a: S);
}

impl<K, S> Scalable<S> for Vector<K>
where
    K: Mul<Output = K> + Copy + Debug,
    S: Into<K> + Copy, 
{
    fn scl(&mut self, a: S) {
        let scalar: K = a.into();
        for x in self.data.iter_mut() {
            *x = *x * scalar;
        }
    }
}

impl<K, S> Scalable<S> for Matrix<K>
where
    K: Mul<Output = K> + Copy + Debug,
    S: Into<K> + Copy,
{
    fn scl(&mut self, a: S) {
        let scalar: K = a.into();
        for row in self.data.iter_mut() {
            for x in row.iter_mut() {
                *x = *x * scalar;
            }
        }
    }
}

impl<K> Vector<K> 
where K: Add<Output = K> + Sub<Output = K> + Copy + Debug 
{
    fn from<T: Into<Vec<K>>>(values: T) -> Self {
        Vector { data: values.into() }
    }
    fn add(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() { self.data[i] = self.data[i] + v.data[i]; }
    }
    fn sub(&mut self, v: &Vector<K>) {
        for i in 0..self.data.len() { self.data[i] = self.data[i] - v.data[i]; }
    }
}

impl<K> Matrix<K> 
where K: Add<Output = K> + Sub<Output = K> + Copy + Debug 
{
    fn from<T : Into<Vec<Vec<K>>>>(values: T) -> Self{
        let data = values.into();
        let _rows = data.len();
        let _cols = if _rows > 0 { data[0].len() } else { 0 };
        Matrix {data, _rows, _cols}
    }
    fn add(&mut self, v: &Matrix<K>){
        for i in 0..self.data.len(){
            for j in 0..self.data[i].len(){ self.data[i][j] = self.data[i][j] + v.data[i][j]; }
        }
    }
    fn sub(&mut self, v: &Matrix<K>){
        for i in 0..self.data.len(){
            for j in 0..self.data[i].len(){ self.data[i][j] = self.data[i][j] - v.data[i][j]; }
        }
    }
}

fn main() {
    let mut v_float = Vector::from(vec![1.5, 2.5, 3.5]);
    v_float.scl(2); 
    println!("Vector f32 scaled by int 2: {:?}", v_float.data);

    let mut m_int = Matrix::from(vec![vec![1, 2], vec![3, 4]]);
    m_int.scl(10); 
    println!("Matrix i32 scaled by int 10: {:?}", m_int.data);
}