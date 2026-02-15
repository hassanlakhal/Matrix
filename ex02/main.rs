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
    rows: usize,
    cols: usize,
}

impl<K> Matrix<K>
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Debug,
{
    fn from<T : Into<Vec<Vec<K>>>>(values: T) -> Self{
        let data = values.into();
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };
        Matrix {data, rows, cols}
    }

    fn add(&mut self, v: &Matrix<K>){
        for i in 0..self.data.len(){
            for j in 0..self.data[i].len(){
                self.data[i][j] = self.data[i][j] + v.data[i][j];
            }
        }
    }
    fn sub(&mut self, v: &Matrix<K>){
        for i in 0..self.data.len(){
            for j in 0..self.data[i].len(){
                self.data[i][j] = self.data[i][j] - v.data[i][j];
            }
        }
    }
    fn scl(&mut self, a: K){
        for i in 0..self.data.len(){
            for j in 0..self.data[i].len(){
                self.data[i][j] = self.data[i][j] * a;
            }
        }
    }
}

impl<K> Vector<K> 
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Copy + Debug,
{
        fn from<T: Into<Vec<K>>>(values: T) -> Self {
            Vector { data: values.into() }
        }

        fn size(&self) -> usize {
            self.data.len()
        }

        fn print(&self) {
            println!("{:?}", self.data);
        }

        fn add(&mut self, v: &Vector<K>) {
            if self.size() != v.size() {
                eprintln!("Error: Vectors must have the same size");
                return;
            }
            for i in 0..self.data.len() {
                self.data[i] = self.data[i] + v.data[i];
            }
        }

        fn sub(&mut self, v: &Vector<K>){
            if self.size() != v.size() {
                eprintln!("Error: Vectors must have the same size");
                return;
            }
            for i in 0..self.data.len() {
                self.data[i] = self.data[i] - v.data[i];
            }
        }
        fn scl(&mut self, a: K){
            for i in 0..self.data.len(){
                self.data[i] = self.data[i] * a;
            }
        }
}


trait Lerpable{
    fn lerp(u: &Self, v: &Self, t: f32) -> Self;
}


impl<K> Lerpable for Matrix<K> 
where 
    K: Add<Output = K> + Sub<Output = K> + Mul<f32, Output = K> + Copy + Debug 
{
    fn lerp(u: &Matrix<K>, v: &Matrix<K>, t: f32) -> Matrix<K> {
        let mut res = v.clone();
        res
    }
}

fn lerp<V: Lerpable>(u:V , v:V, t: f32) -> V{
    V::lerp(&u, &v,t)
}

fn main() {
    let m1 = Matrix::from(vec![vec![2., 1.], vec![3., 4.]]);
    let m2 = Matrix::from(vec![vec![20., 10.], vec![30., 40.]]);

    let mid = lerp(m1, m2, 0.5);

    println!("{:?}", mid);
}