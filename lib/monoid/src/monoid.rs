use std::fmt;
use std::ops::Mul;

pub trait Monoid<RHS = Self>: Mul<RHS, Output = Self> {
    fn e() -> Self;
}

#[derive(Clone, Copy)]
pub struct Integer {
    value: i32,
}

#[derive(Clone, Copy)]
pub enum Float {
    Float(f32),
}

impl Mul for Float {
    type Output = Float;
    fn mul(self, other: Self) -> Float {
        let Float::Float(a) = self;
        let Float::Float(b) = other;
        Float::Float(a * b)
    }
}

impl Monoid for Float {
    fn e() -> Float {
        Float::Float(1.0)
    }
}

impl Mul for Integer {
    type Output = Integer;
    fn mul(self, other: Self) -> Integer {
        Integer {
            value: self.value * other.value,
        }
    }
}

impl Integer {
    pub fn new(value: i32) -> Integer {
        Integer { value }
    }
}

impl Monoid for Integer {
    fn e() -> Integer {
        Integer { value: 1 }
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Int({})", self.value)
    }
}

impl fmt::Display for Float {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Float::Float(v) = *self;
        write!(f, "Float({})", v)
    }
}
