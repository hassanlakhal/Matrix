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

    fn norm_1(&mut self) -> K
    where 
        K : Add<Output = K> + Mul<Output = K> + Default 
    {
        let mut res = K::default();
        let mut vec = self.data.clone();
        for i in 0..vec.len(){
            if vec[i] < 0.{
                vec[i] = vec[i] * -1.;
            }
            res += vec[i];
        }
        res
    }
    // fn norm(&mut self) -> K{

    // }
    // fn norm_inf(&mut self) -> K{

    // }
}


fn main() {
    let mut u = Vector::from(vec![0., 0., 0.]);
    println!("{:?}",u.norm_1());
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 0.0, 0.0, 0.0
    // let u = Vector::from(vec![1., 2., 3.]);
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // // 6.0, 3.74165738, 3.0
    // let u = Vector::from(vec![-1., -2.]);
    // println!("{}, {}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0
}