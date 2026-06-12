#[derive(Debug)]

pub struct Vector<K>{
    pub data: Vec<K>,
}

impl<K> Vector<K>{
    pub fn new(data: Vec<K>) -> Self{
    
        Vector{ data }
    }
}