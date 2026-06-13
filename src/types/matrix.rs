use super::field::Field;

#[derive(Debug, Clone, PartialEq)]

pub struct Matrix<K: Field>{
    pub data: Vec<Vec<K>>,
    pub rows: usize,
    pub cols: usize

}

impl<K: Field> Matrix<K>{
    pub fn form(data: Vec<Vec<K>>) -> Self{
        let rows = data.len();
        let cols = if rows == 0 {0} else {data[0].len()};

        assert!(
            data.iter().all(|r| r.len() == cols),
            "All rows must have the same number of columns"
        );

        Self{ data ,rows , cols}
    }
}