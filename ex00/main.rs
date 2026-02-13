// #[allow(dead_code)]
#[derive(Debug)]
struct Matrix<K> {
    data: Vec<K>,
    rows: usize,
    cols: usize,
}

fn main() {
    let m = Matrix {
        data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
        rows: 2,
        cols: 3,
    };
    
    println!("{:?}", m);
}
