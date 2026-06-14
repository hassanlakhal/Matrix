use matrix::{Vector, Matrix};

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
}