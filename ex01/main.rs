use std::fmt::Debug;

#[derive(Debug, Clone)]
struct Vector {
    data: Vec<f32>,
}

impl Vector {
    fn from(values: &[f32]) -> Self {
        Vector { data: values.to_vec() }
    }

    fn size(&self) -> usize {
        self.data.len()
    }

    fn zero(size: usize) -> Self {
        Vector { data: vec![0.0; size] }
    }
}

fn linear_combination(u: &[Vector], coefs: &[f32]) -> Vector {
    assert!(u.len() == coefs.len(), "Vectors and coefficients must match");
    if u.is_empty() {
        return Vector::zero(0);
    }

    let size = u[0].size();
    let mut result = Vector::zero(size);

    for (vec, &coef) in u.iter().zip(coefs.iter()) {
        for i in 0..size {
            result.data[i] = vec.data[i].mul_add(coef, result.data[i]);
        }
    }

    result
}

fn main() {
    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);

    let vectors = [e1, e2, e3];
    let coefs = [10., -2., 0.5];

    let sum = linear_combination(&vectors, &coefs);
    println!("Linear combination result: {:?}", sum);

    let v1 = Vector::from(&[1., 2., 3.]);
    let v2 = Vector::from(&[0., 10., -100.]);
    let sum2 = linear_combination(&[v1, v2], &[10., -2.]);
    println!("Linear combination result: {:?}", sum2);
}
