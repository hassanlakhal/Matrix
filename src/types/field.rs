use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign};
use std::fmt::Display;

pub trait Field:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + Copy
    + Clone
    + Default
    + PartialEq
    + Display
{
    fn zero() -> Self;
    fn one() -> Self;
 
    fn fma(self, b: Self, c: Self) -> Self;
 
    fn abs(self) -> Self;
 
    fn sqrt(self) -> Self;
}


impl Field for f32{

    fn zero() -> Self{0.0}
    fn one() -> Self {1.0}
    fn fma(self, b: Self, c: Self) -> Self{
        f32::mul_add(self, b, c)
    }

    fn abs(self) -> Self{
        f32::abs(self)
    }

    fn sqrt(self) -> Self{
        self.powf(0.5)
    }
}