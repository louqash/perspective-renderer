use crate::common::*;
#[derive(Clone, Debug)]
pub struct Sphere {
    center: V3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: V3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius,
        }
    }
}

impl<'a> IntoIterator for &'a Sphere {
    type Item = (V3, V3);
    type IntoIter = SphereIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SphereIterator {
            sphere: self,
            phi: (0.0, 0.0),
            dphi: (0.01, 0.01)
        }
    }
}

pub struct SphereIterator<'a> {
    sphere: &'a Sphere,
    phi: (f32, f32),
    dphi: (f32, f32),
}

impl<'a> Iterator for SphereIterator<'a> {
    type Item = (V3, V3);
    fn next(&mut self) -> Option<Self::Item> {
        if self.phi.0 < std::f32::consts::PI * 2.0 {
            if self.phi.1 < std::f32::consts::PI * 2.0 {
                let rx = self.sphere.radius * self.phi.1.cos();
                let ry = self.sphere.radius * self.phi.1.sin();
                let rz = 0.0;

                let phi_0_cos = self.phi.0.cos();
                let phi_0_sin = self.phi.0.sin();

                let x = rx*phi_0_cos - rz*phi_0_sin;
                let z = rx*phi_0_sin + rz*phi_0_cos;
                let y = ry;

                self.phi.1 += self.dphi.1;
                return Some((V3(x, y, z), V3(x, y, z)));
            } else {
                self.phi.1 = 0.0;
                self.phi.0 += self.dphi.0;
                return self.next();
            }

        } else {
            None
        }
    }
}
