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

    fn norm_1(&mut self) -> f32{

    }
    fn norm(&mut self) -> f32{

    }
    fn norm_inf(&mut self) -> f32{

    }
}


fn main() {
    let u = Vector::from(vec![0., 0., 0.]);
    println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let u = Vector::from(vec![1., 2., 3.]);
    println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from(vec![-1., -2.]);
    println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}