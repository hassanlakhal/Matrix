// use super::field::Field;
// use std::fmt;

pub trait Lerp:{
    fn lerp(u: Self, v: Self, t: f32) -> Self;
}

impl Lerp for f32{
    fn lerp(u: Self, v: Self, t: f32) -> Self{
        f32::mul_add(t, v - u, u)
    }
}

pub fn lerp<V: Lerp>(u: V, v: V, t: f32) -> V {
    V::lerp(u, v, t)
}
