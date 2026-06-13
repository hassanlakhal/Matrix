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
}

impl<K: Field> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for val in &self.data {
            writeln!(f, "[{}]", val)?;
        }
        Ok(())
    }
}