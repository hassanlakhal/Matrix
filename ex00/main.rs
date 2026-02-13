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

fn main() {
    let mut u = Vector::from([2., 3., 5.]);
    let v = Vector::from([5., 7., -100.]);
    let mut j = Vector::from([8., 9.]);

    println!("Before add:");
    u.print();  

    u.add(&v);


    println!("After add:");
    u.print();

    j.sub(&v);
    println!("After sub:");
    j.print();
    j.scl(5.);
    j.print();


    let mut u1 = Matrix::from(vec![vec![1., 2.], vec![3., 4.]]);
    let mut u2 = Matrix::from(vec![vec![1., 0.], vec![8., 7.]]);
    let mut u3 = Matrix::from(vec![vec![1., 0.], vec![8., 7.]]);
    let v1 = Matrix::from(vec![vec![7., 4.], vec![-2., 2.]]);
    u1.add(&v1);
    u2.sub(&v1);
    u3.scl(-1.);
    println!("Rows: {}, Cols: {}", u1.rows, u1.cols);
    println!("Data: {:?}", u1.data);
    println!("=================");
    println!("Data: {:?}", u2.data);
    println!("=================");
    println!("Data: {:?}", u3.data);

}