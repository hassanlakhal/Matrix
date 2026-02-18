use std::ops::{AddAssign, SubAssign, MulAssign,DivAssign, Mul, Add, Div, Sub};
use std::cmp::{PartialOrd};
use std::fmt::{Display};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

pub trait KField: 
    Copy + Default + PartialEq + Display +
    Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign 
{}

impl KField for f32 {}


impl<K: KField> Vector<K> {
    pub fn from<const N: usize>(arr: [K; N]) -> Self {
        Self { data: arr.to_vec() }
    }
}

trait MathOps{
    fn pow(self, exp: u32) -> Self;
    fn powf(self, exp: f32) -> Self;
}

impl MathOps for f32 {
    
    fn pow(self, exp: u32) -> Self {
        f32::powf(self, exp as f32) as f32
    }

    fn powf(self, exp: f32) -> Self {
        f32::powf(self, exp as f32) as f32
    }

}


impl<K> Vector<K> 
where K: Copy + AddAssign + SubAssign + MulAssign + PartialOrd
{
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

    fn norm(&self) -> K
    where
        K: Add<Output = K> + Mul<Output = K> + MathOps + Default
    {
        let vec = self.data.clone();
        let mut res = K::default();
        for i in 0..self.data.len(){
            
            res = res + vec[i].pow(2);
        }
        res = res.powf(0.5);
        res
    }

}

fn angle_cos<K>(u: & Vector<K>, v: & Vector<K>) -> f32
where
    K: Copy + AddAssign + SubAssign + MulAssign + PartialOrd + Default + MathOps
     + Mul<Output = K> + Add<Output = K> + Div<Output = K> + Into<f32>
{
    
    let dot = u.dot(v).into();
    let norm_v = v.norm().into();
    let norm_u = u.norm().into();

    let angle_cos = dot / (norm_u * norm_v);
    
    angle_cos
}

fn main() {
    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([ 1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let v = Vector::from([1., 2., 3.]);
    let u = Vector::from([4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846
}