use std::ops::{AddAssign, SubAssign, MulAssign};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

#[derive(Debug, Clone)]
struct Matrix<K> {
    data: Vec<K>,   // Row-major storage
    rows: usize,
    cols: usize,
}


impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}

impl<K> Vector<K>
where
    K: Copy + AddAssign + SubAssign + MulAssign,
{
    pub fn from(values: Vec<K>) -> Self {
        Self { data: values }
    }

    pub fn add(&mut self, v: &Vector<K>) {
        assert!(self.size() == v.size(), "Vector dimensions must match");
        for i in 0..self.size() {
            self.data[i] += v.data[i];
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        assert!(self.size() == v.size(), "Vector dimensions must match");
        for i in 0..self.size() {
            self.data[i] -= v.data[i];
        }
    }

    pub fn scl(&mut self, a: K) {
        for val in &mut self.data {
            *val *= a;
        }
    }
}



impl<K> Matrix<K> {
    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_square(&self) -> bool {
        self.rows == self.cols
    }

    // Row-major index helper
    fn index(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

impl<K> Matrix<K>
where
    K: Copy + AddAssign + SubAssign + MulAssign,
{
    pub fn from(values: Vec<Vec<K>>) -> Self {
        let rows = values.len();
        let cols = if rows > 0 { values[0].len() } else { 0 };

        let mut data = Vec::with_capacity(rows * cols);

        for row in &values {
            assert!(row.len() == cols, "All rows must have same length");
            for val in row {
                data.push(*val);
            }
        }

        Self { data, rows, cols }
    }

    pub fn add(&mut self, m: &Matrix<K>) {
        assert!(self.shape() == m.shape(), "Matrix dimensions must match");
        for i in 0..self.size() {
            self.data[i] += m.data[i];
        }
    }

    pub fn sub(&mut self, m: &Matrix<K>) {
        assert!(self.shape() == m.shape(), "Matrix dimensions must match");
        for i in 0..self.size() {
            self.data[i] -= m.data[i];
        }
    }

    pub fn scl(&mut self, a: K) {
        for val in &mut self.data {
            *val *= a;
        }
    }
}


fn main() {
    // Vector
    let mut v1 = Vector::from(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::from(vec![4.0, 5.0, 6.0]);

    v1.add(&v2);
    println!("Vector after add: {:?}", v1);

    v1.scl(2.0);
    println!("Vector after scale: {:?}", v1);

    // Matrix
    let mut m1 = Matrix::from(vec![
        vec![1.0, 2.0],
        vec![3.0, 4.0],
    ]);

    let m2 = Matrix::from(vec![
        vec![5.0, 6.0],
        vec![7.0, 8.0],
    ]);

    m1.add(&m2);
    println!("Matrix after add: {:?}", m1);

    m1.scl(2.0);
    println!("Matrix after scale: {:?}", m1);

    println!("Shape: {:?}", m1.shape());
    println!("Is square? {}", m1.is_square());
}
