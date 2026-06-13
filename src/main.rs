use matrix::{Vector, Matrix};

fn main(){
    let mut u = Vector::from(vec![2.0, 3.0]);
    let v = Vector::from(vec![5.0, 7.0]);
    u.add(v);
    println!("{}", u);
    let mut u = Vector::from([2.0, 3.0]);
    let v = Vector::from([5.0, 7.0]);
    u.sub(v);
    println!("{}", u);
    let mut u = Vector::from([2., 0.3]);
    u.scl(2.);
    println!("{}", u);
}