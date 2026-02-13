#[derive(Debug, Clone)]
struct Vector<K> {
    data: Vec<K>,
}

// #[derive(Debug, Clone)]
// struct Matrix<K> {
//     data: Vec<K>,
//     rows: usize,
//     cols: usize,
// }

impl<K: std::fmt::Debug> Vector<K>{

    fn size(&self) -> usize{
        self.data.len()
    }

    fn print(&self){
        println!("{:?}",self.data);
    }
}


fn main() {

    let v = Vector { data: vec![1.0,2.0,3.0,4.0,5.0,6.0] };
    v.print();
    println!("Vector size: {}", v.size());
}

