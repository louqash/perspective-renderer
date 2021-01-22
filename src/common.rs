use std::ops::{Add, Sub, Mul, AddAssign, Div};
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
pub struct V3(pub f32, pub f32, pub f32);

impl V3 {
    pub fn len(&self) -> f32 {
        self.dot(self).sqrt()
    }
    pub fn dot(&self, other: &V3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl PartialEq for V3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 &&
        self.1 == other.1 &&
        self.2 == other.2
    }
}
impl PartialOrd for V3 {
    fn partial_cmp(&self, other: &V3) -> Option<Ordering> {
        if self.0 < other.0 || self.1 < other.1 || self.2 < other.2 {
            Some(Ordering::Less)
        } else if self == other {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl AddAssign<V3> for V3 {
    fn add_assign(&mut self, rhs: V3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl Add<V3> for V3 {
    type Output = V3;
    fn add(self, rhs: V3) -> Self::Output {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<&V3> for V3 {
    type Output = V3;
    fn add(self, rhs: &V3) -> Self::Output {
        V3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
impl Add<f32> for V3 {
    type Output = V3;
    fn add(self, rhs: f32) -> Self::Output {
        V3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl Sub<&V3> for V3 {
    type Output = V3;
    fn sub(self, rhs: &V3) -> Self::Output {
        V3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}
impl Sub<f32> for V3 {
    type Output = V3;
    fn sub(self, rhs: f32) -> Self::Output {
        V3(self.0 - rhs, self.1 - rhs, self.2 - rhs)
    }
}

impl Mul<f32> for V3 {
    type Output = V3;
    fn mul(self, rhs: f32) -> Self::Output {
        V3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Div<f32> for V3 {
    type Output = V3;
    fn div(self, rhs: f32) -> Self::Output {
        V3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
