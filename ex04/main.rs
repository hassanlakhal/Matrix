use std::ops::{AddAssign, SubAssign, MulAssign, Mul, Add, Neg};
use std::cmp::{PartialOrd};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}


impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

trait MathOps{
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn pow(self, exp: u32) -> Self;
    fn max(self, other: Self) -> Self; 
}

impl MathOps for f32 {
    fn mul_add(self, a: Self, b: Self) -> Self {
        f32::mul_add(self, a, b)
    }

    fn pow(self, exp: u32) -> Self {
        f32::powf(self, exp as f32) as f32
    }

    fn max(self, other: Self) -> Self {
        f32::max(self, other)
    }
}


impl<K> Vector<K> 
where K: Copy + AddAssign + SubAssign + MulAssign + PartialOrd
{
    pub fn from(values: Vec<K>) -> Self {
        Self { data: values }
    }

    fn norm_1(&mut self) -> K
    where 
        K : Add<Output = K> + Mul<Output = K> + Neg<Output = K> + Default 
    {
        let mut res = K::default();
        let abs = K::default();
        let mut vec = self.data.clone();
        for i in 0..self.size(){
            if vec[i] < abs{
                vec[i] = -vec[i];
            }
            res += vec[i];
        }
        res
    }
    fn norm(&mut self) -> K
    where
        K: Add<Output = K> + Mul<Output = K> + MathOps + Default
    {
        let vec = self.data.clone();
        let mut res = K::default();
        for i in 0..self.size(){
            
            res = res + vec[i].pow(2);
        }
        res = res.pow(1.5);
        res
    }
    // fn norm_inf(&mut self) -> K{

    // }
}


fn main() {
    let mut u = Vector::from(vec![0., 0., 0.]);
    println!("{:?}",u.norm());
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 0.0, 0.0, 0.0
    let mut u = Vector::from(vec![1., 2., 3.]);
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    println!("{:?}", u.norm());
    // // 6.0, 3.74165738, 3.0
    let mut u = Vector::from(vec![-1., -2.]);
    println!("{:?}", u.norm());
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}