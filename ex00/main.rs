#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

// #[derive(Debug, Clone)]
// struct Matrix<K> {
//     data: Vec<K>,
//     rows: usize,
//     cols: usize,
// }

use std::{
    ops::{Add, Sub},
    fmt::Debug,
};


impl<K> Vector<K> 
where
    K: Add<Output = K> + Sub<Output = K> + Copy + Debug,
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
    // fn scl...
}

fn main() {
    let mut u = Vector::from([2., 3.]);
    let v = Vector::from([5., 7.]);
    let mut j = Vector::from([8., 9.]);

    println!("Before add:");
    u.print();  

    u.add(&v);


    println!("After add:");
    u.print();

    j.sub(&v);
    println!("After sub:");
    j.print();
}