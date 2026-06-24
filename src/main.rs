use matrix::{Vector, Matrix, linear_combination, lerp, angle_cos, cross_product, projection};

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


    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("{}", lerp(21., 42., 0.3));
    // 27.3

    println!("{}", lerp(Vector::from([2., 1.]), Vector::from([4., 2.]), 0.3));
    // // [2.6]
    // // [1.3]
    println!("{}", lerp(Matrix::from([[2., 1.], [3., 4.]]), Matrix::from([[20.,
    10.], [30., 40.]]), 0.5));
    // [[11., 5.5]
    // [16.5, 22.]]

    let  u = Vector::from([0., 0.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(v));
    // 0.0
    let u = Vector::from([1., 1.]);
    let v = Vector::from([1., 1.]);
    println!("{}", u.dot(v));
    // 2.0
    let u = Vector::from([-1., 6.]);
    let v = Vector::from([3., 2.]);
    println!("{}", u.dot(v));
    // 9.0

    let u = Vector::from([0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    // println!("{}",u.norm_1());
    let u = Vector::from([1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // println!("{}",u.norm_1());
    // 6.0, 3.74165738, 3.0
    let u = Vector::from([-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // println!("{}",u.norm_1());
    // 3.0, 2.236067977, 2.0


    let u = Vector::from([1., 0.]);
    let v = Vector::from([1., 0.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 0.]);
    let v = Vector::from([0., 1.]);
    println!("{}", angle_cos(&u, &v));
    // 0.0
    let u = Vector::from([-1., 1.]);
    let v = Vector::from([ 1., -1.]);
    println!("{}", angle_cos(&u, &v));
    // -1.0
    let u = Vector::from([2., 1.]);
    let v = Vector::from([4., 2.]);
    println!("{}", angle_cos(&u, &v));
    // 1.0
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", angle_cos(&u, &v));
    // 0.974631846


    let u = Vector::from([0., 0., 1.]);
    let v = Vector::from([1., 0., 0.]);
    println!("{}", cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from([1., 2., 3.]);
    let v = Vector::from([4., 5., 6.]);
    println!("{}", cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from([4., 2., -3.]);
    let v = Vector::from([-2., -5., 16.]);
    println!("{}", cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]

    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(v));
    // [4.]
    // [2.]
    let  u = Matrix::from([
    [2., 0.],
    [0., 2.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(v));
    // [8.]
    // [4.]
    let u = Matrix::from([
    [2., -2.],
    [-2., 2.],
    ]);
    let v = Vector::from([4., 2.]);
    println!("{}", u.mul_vec(v));
    // [4.]
    // [-4.]

    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Matrix::from([
    [1., 0.],
    [0., 1.],]);
    println!("{}", u.mul_mat(v));
    // [1., 0.]
    // [0., 1.]
    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    let v = Matrix::from([
    [2., 1.],
    [4., 2.],
    ]);
    println!("{}", u.mul_mat(v));
    // [2., 1.]
    // [4., 2.]
    let u = Matrix::from([
    [3., -5.],
    [6., 8.],
    ]);
    let v = Matrix::from([
    [2., 1.],
    [4., 2.],
    ]);
    println!("{}", u.mul_mat(v));
    // [-14., -7.]
    // [44., 22.]


    let u = Matrix::from([
    [1., 0.],
    [0., 1.],
    ]);
    println!("{}", u.trace());
    // 2.0
    let u = Matrix::from([
    [2., -5., 0.],
    [4., 3., 7.],
    [-2., 3., 4.],
    ]);
    println!("{}", u.trace());
    // 9.0
    let u = Matrix::from([
    [-2., -8., 4.],
    [1., -23., 4.],
    [0., 6., 4.],
    ]);
    println!("{}", u.trace());
    // -21.0
    // let u = Matrix::from([
    // [1., 0.],
    // [0., 1.],
    // [0., -1.],
    // ]);
    println!("{}",u.transpose());

    let mut u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    
    let mut  u = Matrix::from([
    [1., 2.],
    [3., 4.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let mut u = Matrix::from([
    [1., 2.],
    [2., 4.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let mut u = Matrix::from([
    [8., 5., -2., 4., 28.],
    [4., 2.5, 20., 4., -4.],
    [8., 5., 1., 4., 17.],
    ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]

    let mut u = Matrix::from([
    [ 1., -1.],
    [-1., 1.],
    ]);
    println!("{}", u.determinant());
    // 0.0
    let mut u = Matrix::from([
    [2., 0., 0.],
    [0., 2., 0.],
    [0., 0., 2.],
    ]);
    println!("{}", u.determinant());
    // 8.0
    let mut u = Matrix::from([
    [ 8., 5., -2., 4.],
    [ 4., 2.5, 20., 4.],
    [ 8., 5., 1., 4.],
    [28., -4., 17., 1.],
    ]);
    println!("{}", u.determinant());
    // 1032
    let mut u =  Matrix::from([
        [8., 5., -2.],
        [4., 7., 20.],
        [7., 6., 1.],
    ]);
    println!("{}", u.determinant());
    // -174.0


    let mut u = Matrix::from([
        [1., 0., 0.],
        [0., 1., 0.],
        [0., 0., 1.],
    ]);
    match u.inverse() {
        Ok(inv) => println!("Inverse:\n{}", inv),
        Err(e) => println!("Error: {}", e),
    }
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let mut u = Matrix::from([
    [2., 0., 0.],
    [0., 2., 0.],
    [0., 0., 2.],
    ]);
    match u.inverse() {
        Ok(inv) => println!("Inverse:\n{}", inv),
        Err(e) => println!("Error: {}", e),
    }
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let mut u = Matrix::from([
    [8., 5., -2.],
    [4., 7., 20.],
    [7., 6., 1.],
    ]);
    match u.inverse() {
        Ok(inv) => println!("Inverse:\n{}", inv),
        Err(e) => println!("Error: {}", e),
    }
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]


    let mut u = Matrix::from([
    [1., 0., 0.],
    [0., 1., 0.],
    [0., 0., 1.],
    ]);
    println!("{}", u.rank());
    // 3
    let mut u = Matrix::from([
    [ 1., 2., 0., 0.],
    [ 2., 4., 0., 0.],
    [-1., 2., 1., 1.],
    ]);
    println!("{}", u.rank());
    // 2
    let mut u = Matrix::from([
    [ 8., 5., -2.],
    [ 4., 7., 20.],
    [ 7., 6., 1.],
    [21., 18., 7.],
    ]);
    println!("{}", u.rank());
    // 3
    let p = projection(
        std::f32::consts::FRAC_PI_4,  // 45° f radians
        16.0 / 9.0,                    // ratio
        0.1,                           // near
        100.0,                         // far
    );
    println!("{}", p);
}