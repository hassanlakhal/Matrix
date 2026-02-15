use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

#[derive(Debug, Clone)]
struct Matrix<K> {
    data: Vec<Vec<K>>,
    rows: usize,
    cols: usize,
}


impl<K> Matrix<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<f32, Output = K>
        + Copy
        + Debug,
{
    fn from<T: Into<Vec<Vec<K>>>>(values: T) -> Self {
        let data = values.into();
        let rows = data.len();
        let cols = if rows > 0 { data[0].len() } else { 0 };

        Matrix { data, rows, cols }
    }

    fn add(&mut self, v: &Matrix<K>) {
        assert!(self.rows == v.rows && self.cols == v.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = self.data[i][j] + v.data[i][j];
            }
        }
    }

    fn sub(&mut self, v: &Matrix<K>) {
        assert!(self.rows == v.rows && self.cols == v.cols);

        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = self.data[i][j] - v.data[i][j];
            }
        }
    }

    fn scl(&mut self, a: f32) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.data[i][j] = self.data[i][j] * a;
            }
        }
    }
}


impl<K> Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<f32, Output = K>
        + Copy
        + Debug,
{
    fn from<T: Into<Vec<K>>>(values: T) -> Self {
        Vector {
            data: values.into(),
        }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn add(&mut self, v: &Vector<K>) {
        assert!(self.size() == v.size());

        for i in 0..self.size() {
            self.data[i] = self.data[i] + v.data[i];
        }
    }

    fn sub(&mut self, v: &Vector<K>) {
        assert!(self.size() == v.size());

        for i in 0..self.size() {
            self.data[i] = self.data[i] - v.data[i];
        }
    }

    fn scl(&mut self, a: f32) {
        for i in 0..self.size() {
            self.data[i] = self.data[i] * a;
        }
    }
}


trait Lerpable {
    fn lerp(u: &Self, v: &Self, t: f32) -> Self;
}



impl Lerpable for f32 {
    fn lerp(u: &f32, v: &f32, t: f32) -> f32 {
        *u + (*v - *u) * t
    }
}



impl<K> Lerpable for Vector<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<f32, Output = K>
        + Copy
        + Debug,
{
    fn lerp(u: &Vector<K>, v: &Vector<K>, t: f32) -> Vector<K> {
        let mut res = v.clone();
        res.sub(u);
        res.scl(t);
        res.add(u);
        res
    }
}



impl<K> Lerpable for Matrix<K>
where
    K: Add<Output = K>
        + Sub<Output = K>
        + Mul<f32, Output = K>
        + Copy
        + Debug,
{
    fn lerp(u: &Matrix<K>, v: &Matrix<K>, t: f32) -> Matrix<K> {
        let mut res = v.clone();
        res.sub(u);
        res.scl(t);
        res.add(u);
        res
    }
}


fn lerp<V: Lerpable>(u: V, v: V, t: f32) -> V {
    V::lerp(&u, &v, t)
}


fn main() {
    println!("{:?}", lerp(0.0, 1.0, 0.0));
    println!("{:?}", lerp(0.0, 1.0, 1.0));

    let v1 = Vector::from(vec![1.0, 2.0]);
    let v2 = Vector::from(vec![3.0, 4.0]);

    let vmid = lerp(v1, v2, 0.5);
    println!("{:?}", vmid);

    let m1 = Matrix::from(vec![vec![2.0, 1.0], vec![3.0, 4.0]]);
    let m2 = Matrix::from(vec![vec![20.0, 10.0], vec![30.0, 40.0]]);

    let mid = lerp(m1, m2, 0.5);
    println!("{:?}", mid);
}
