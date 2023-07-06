use std::{fmt::Display, ops::Add};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
struct Complex<T> {
    re: T,
    im: T,
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Self {
        Complex { re, im }
    }
}

impl<T: Add<T, Output = T>> Add for Complex<T> {
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(value: (T, T)) -> Self {
        Complex {
            re: value.0,
            im: value.1,
        }
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn complex_basics() {
        let first = Complex::new(3, 5);
        let second: Complex<i32> = Complex::default();
        println!("{:?}", first + second);
    }

    #[test]
    fn complex_addition() {
        let a = Complex::new(1, -2);
        let b = Complex::default();
        let res = a + b;
        assert_eq!(res, a);
    }

    #[test]
    fn complex_from() {
        let a = (234, 456);
        let comp = Complex::from(a);
        assert_eq!(comp.re, a.0);
        assert_eq!(comp.im, a.1);
    }

    #[test]
    fn complex_display() {
        let c = Complex::new(234, 456);
        println!("{}", c);
    }
}
