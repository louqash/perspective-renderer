use std::ops::{Add, Sub};

#[derive(Clone, Debug)]
pub struct V3(pub f32, pub f32, pub f32);

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

pub trait RenderObject {
    fn get_next(&self) -> Option<V3>;
}
