use std::ops::{AddAssign, SubAssign, MulAssign, Mul, Add};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}


impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}


impl<K> Vector<K> 
where K: Copy + AddAssign + SubAssign + MulAssign 
{
    pub fn from(values: Vec<K>) -> Self {
        Self { data: values }
    }

    fn dot (&self, v: Vector<K>) -> K
    where 
        K: Mul<Output = K> + Add<Output = K> + Default
    {
        let mut res = K::default();
        if self.size() != v.size() {return res}
        for i in 0..self.size() {
            res = self.data[i] * v.data[i] + res;
        }
        res
    }
}


fn main() {
    let u = Vector::from(vec![0., 0. , 100.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{:?}", u.dot(v));
    // 0.0
    let u = Vector::from(vec![1., 1.]);
    let v = Vector::from(vec![1., 1.]);
    println!("{:?}", u.dot(v));
    // 2.0
    let u = Vector::from(vec![-1., 6.]);
    let v = Vector::from(vec![3., 2.]);
    println!("{:?}", u.dot(v));
    // 9.0
}