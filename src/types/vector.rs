// use std::fmt;
use super::field::Field;


#[derive(Debug, Clone,PartialEq,)]

pub struct Vector<K : Field>{
    pub data: Vec<K>,
}

impl<K: Field> Vector<K>{
    pub fn form(data:impl Into<Vec<K>>) -> Self{
    
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
}