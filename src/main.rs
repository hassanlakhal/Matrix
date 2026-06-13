use matrix::{Vector, Matrix};

fn main(){
    let new_vector = Vector::form(vec![1.0,1.5]);
    let new_matrix = Matrix::new(vec![vec![1.0,0.0,0.2], vec![1.0,1.2,5.0]]);
    println!("Vector: {:?}",new_vector);
    println!("Vector: {:?}",new_matrix);
}