use std::fmt;
use super::field::Field;


#[derive(Debug, Clone,PartialEq,)]

pub struct Vector<K : Field>{
    pub data: Vec<K>,
}

impl<K: Field> Vector<K>{
    pub fn from(data:impl Into<Vec<K>>) -> Self{
    
        Self { data: data.into()}
    }
    pub fn size(&self) -> usize{
        self.data.len()
    }

    pub fn assert_same_size(&self, other: &Self) {
        assert_eq!(
            self.size(), other.size(),
            "Vector size mismatch: {} vs {}",
            self.size(), other.size()
        );
    }

    pub fn add(&mut self, v: Vector<K>){
        for i in 0..self.data.len(){
            self.data[i] += v.data[i]
        }
    }

    pub fn sub(&mut self, v: Vector<K>){
        for i in 0..self.data.len(){
            self.data[i] -= v.data[i]
        }
    }
    
    pub fn scl(&mut self, a: K){
        for i in 0..self.data.len(){
            self.data[i] *= a
        }
    }
    
    pub fn dot(&mut self, v: Vector::<K>) -> K{
        let mut result = K::zero();

        for i in 0..self.data.len(){
            result += self.data[i] * v.data[i]; 
        }

        result
    }
}

pub fn linear_combination<K : Field>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>{
    let mut reusult = Vector::from(vec![K::zero(); u[0].data.len()]);

    for i in 0..u.len(){
        let mut scl_vectot = u[i].clone();
        scl_vectot.scl(coefs[i]);
        reusult.add(scl_vectot);
    }

    reusult
}



impl<K: Field> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.data {
            writeln!(f, "[{}]", val)?;
        }
        Ok(())
    }
}