use matrix::{Vector, Matrix, linear_combination};

fn main(){
    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);
    u.add(v);
    println!("{}", u);
    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);
    u.sub(v);
    println!("{}", u);
    let mut u = Vector::from([2., 0.3]);
    u.scl(2.);
    println!("{}", u);

    let mut u = Matrix::from(vec![
    vec![1., 2.],
    vec![3., 4.]
    ]);
    let v = Matrix::from([
    [7., 4.],
    [-2., 2.]
    ]);
    u.add(v);
    println!("{}", u);

    let mut u = Matrix::from([
    [1., 2.],
    [3., 4.]
    ]);
    let v = Matrix::from([
    [7., 4.],
    [-2., 2.]
    ]);
    u.sub(v);
    println!("{}", u);
    // [-6.0, -2.0]
    // [5.0, 2.0]
    let mut u = Matrix::from([
    [1., 2.],
    [3., 4.]
    ]);
    u.scl(2.);
    println!("{}", u);
    // [2.0, 4.0]
    // [6.0, 8.0]

    let e1 = Vector::from([1., 0., 0.]);
    let e2 = Vector::from([0., 1., 0.]);
    let e3 = Vector::from([0., 0., 1.]);
    let v1 = Vector::from([1., 2., 3.]);
    let v2 = Vector::from([0., 10., -100.]);
    println!("{}", linear_combination(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]

}