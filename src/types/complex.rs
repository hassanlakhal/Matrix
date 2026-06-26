use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign};
use super::field::Field;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Complex {
    pub re: f32,
    pub im: f32,
}

impl Complex {
    pub fn from(re: f32, im: f32) -> Self {
        Self { re, im }
    }

    pub fn modulus(self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from(self.re + rhs.re, self.im + rhs.im)
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from(self.re - rhs.re, self.im - rhs.im)
    }
}

impl Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::from(
            self.re * rhs.re - self.im * rhs.im,
            self.re * rhs.im + self.im * rhs.re,
        )
    }
}

impl Div for Complex {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self::from(
            (self.re * rhs.re + self.im * rhs.im) / denom,
            (self.im * rhs.re - self.re * rhs.im) / denom,
        )
    }
}

impl Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Self::from(-self.re, -self.im)
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}
impl MulAssign for Complex {
    fn mul_assign(&mut self, rhs: Self) { *self = *self * rhs; }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{}+{}i", self.re, self.im)
        } else {
            write!(f, "{}{}i", self.re, self.im)
        }
    }
}

impl Into<f32> for Complex {
    fn into(self) -> f32 {
        self.modulus()
    }
}

impl Field for Complex {
    fn zero() -> Self { Self::from(0.0, 0.0) }
    fn one() -> Self { Self::from(1.0, 0.0) }
    fn fma(self, b: Self, c: Self) -> Self { self * b + c }
    fn abs(self) -> Self {
        Self::from(self.modulus(), 0.0)
    }
    fn sqrt(self) -> Self {
        let r = self.modulus().sqrt();
        let theta = self.im.atan2(self.re) / 2.0;
        Self::from(r * theta.cos(), r * theta.sin())
    }
}