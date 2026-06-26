use super::field::Field;
use crate::Matrix;
use crate::Vector;

pub trait Lerp:{
    fn lerp(u: Self, v: Self, t: f32) -> Self;
}

impl Lerp for f32{
    fn lerp(u: Self, v: Self, t: f32) -> Self{
        f32::mul_add(t, v - u, u)
    }
}

impl<K: Field + std::convert::From<f32> + PartialOrd> Lerp for Matrix<K>{
    fn lerp(u: Self, mut v: Self, t: f32) -> Self{

        let t_generic: K = t.into();
        v.sub(u.clone());
        v.scl(t_generic);
        v.add(u.clone());
        v
    }

}

impl<K: Field + std::convert::From<f32>> Lerp for Vector<K>{
    fn lerp(u: Self, mut v: Self, t: f32) -> Self{
        
        let t_generic: K = t.into();
        v.sub(u.clone());
        v.scl(t_generic);
        v.add(u.clone());
        v
    }

}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    V::lerp(u, v, t)
}
