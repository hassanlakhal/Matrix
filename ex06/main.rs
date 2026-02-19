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
    fn size(&self) -> usize
    where
        K: Add<Output = K> + Mul<Output = K> + MathOps + Default
    {
        let size; 
        size = self.data.len();
        size
    }

}

fn cross_product<K, KField>(u: &Vector::<K>, v: &Vector::<K>) -> Vector<K>
where
    K: Copy + AddAssign + SubAssign + MulAssign + PartialOrd + Default + MathOps
     + Mul<Output = K> + Add<Output = K> + Div<Output = K> + Into<f32> 

{

    if u.size() != 3. && v.size() != 3{
        return Vector::from([0., 0., 0.]);
    }

    let mut cross_v = Vector::from([0., 0. , 0.]);

    cross_v[0] = u[1] * v[2] - u[2] * v[1];
    cross_v[1] = u[2] - v[0] - u[0] * v[2];
    cross_v[2] = u[0] - v[1] - u[1] * v[0];

    cross_v
}
fn main(){

    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]

}