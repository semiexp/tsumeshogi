use std::ops::{Add, Mul, Sub};

/// Type for positions on the Shogi board.
/// P(y, x) represents the cell at the (y+1)-th row from the top and
/// the (x+1)-th column from the left.
/// Note that this notation is different from the common way used in Shogi.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct P(pub i32, pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct D(pub i32, pub i32);

impl P {
    pub fn y(self) -> i32 {
        self.0
    }
    pub fn x(self) -> i32 {
        self.1
    }
}
impl D {
    pub fn flip(self) -> D {
        D(-self.0, self.1)
    }
    pub fn flip_if(self, cond: bool) -> D {
        D(if cond { -self.0 } else { self.0 }, self.1)
    }
}
impl Add<D> for P {
    type Output = P;
    fn add(self, rhs: D) -> P {
        P(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<D> for P {
    type Output = P;
    fn sub(self, rhs: D) -> P {
        P(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Sub<P> for P {
    type Output = D;
    fn sub(self, rhs: P) -> D {
        D(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Add<D> for D {
    type Output = D;
    fn add(self, rhs: D) -> D {
        D(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<D> for D {
    type Output = D;
    fn sub(self, rhs: D) -> D {
        D(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Mul<i32> for D {
    type Output = D;
    fn mul(self, rhs: i32) -> D {
        D(self.0 * rhs, self.1 * rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positions() {
        assert_eq!(P(1, 2) + D(3, 0), P(4, 2));
        assert_eq!(P(1, 2) - D(3, 0), P(-2, 2));
        assert_eq!(D(1, 2) + D(3, 0), D(4, 2));
        assert_eq!(D(1, 2) - D(3, 0), D(-2, 2));
        assert_eq!(D(1, 2) * 4, D(4, 8));
    }
}
