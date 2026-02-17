use std::fmt;
use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};

pub trait KField: 
    Copy + Default + PartialEq + fmt::Display +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign 
{}

impl KField for f32 {}

pub struct Vector<K> { pub data: Vec<K> }


impl<K: KField> Vector<K> {
    pub fn from<const N: usize>(arr: [K; N]) -> Self {
        Self { data: arr.to_vec() }
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

impl<K: KField> Vector<K> {
    fn dot (&self, v: Vector<K>) -> K
    {
        let mut res = K::default();
        if self.data.len() != v.data.len() {return res}
        for i in 0..self.data.len() {
            res = self.data[i] * v.data[i] + res;
        }
        res
    }
}


fn main() {
    let u = Vector::from([0., 0. , 100.]);
    let v = Vector::from([1., 1.]);
    println!("{:?}", u.dot(v));
    // 0.0
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{:?}", u.dot(v));
    // 2.0
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{:?}", u.dot(v));
    // 9.0
}