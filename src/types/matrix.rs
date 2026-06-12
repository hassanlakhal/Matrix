#[derive(Debug)]

pub struct Matrix<K>{
    pub data: Vec<Vec<K>>,
}

impl<K> Matrix<K>{
    pub fn new(data: Vec<Vec<K>>) -> Self{
        Matrix{ data }
    }
}