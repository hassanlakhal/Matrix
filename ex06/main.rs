use std::ops::{
    Add, Sub, Mul, Div,
    AddAssign, SubAssign, MulAssign, DivAssign,
    Index, IndexMut
};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

pub trait KField:
    Copy + Default + PartialEq + Display +
    Add<Output = Self> + Sub<Output = Self> +
    Mul<Output = Self> + Div<Output = Self> +
    AddAssign + SubAssign + MulAssign + DivAssign
{}

impl KField for f32 {}

impl<K: KField> Vector<K> {
    pub fn from<const N: usize>(arr: [K; N]) -> Self {
        Self { data: arr.to_vec() }
    }

    fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<K: Display> Display for Vector<K> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for val in &self.data {
            writeln!(f, "[{}]", val)?;
        }
        Ok(())
    }
}

fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: KField
{
    if u.size() != 3 || v.size() != 3 {
        let zero = K::default();
        return Vector::from([zero, zero, zero]);
    }

    let mut result = Vector::from([K::default(); 3]);

    result[0] = u[1] * v[2] - u[2] * v[1];
    result[1] = u[2] * v[0] - u[0] * v[2];
    result[2] = u[0] * v[1] - u[1] * v[0];

    result
}

fn main() {
    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));

    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));

    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
}