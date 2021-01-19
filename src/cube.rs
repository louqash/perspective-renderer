use crate::common::*;
#[derive(Clone, Debug)]
pub struct Cube {
    center: V3,
    edge_length: f32,
}

impl Cube {
    pub fn new(center: V3, edge_length: f32) -> Cube {
        Cube {
            center,
            edge_length,
        }
    }
}

impl<'a> IntoIterator for &'a Cube {
    type Item = (V3, V3);
    type IntoIter = CubeIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CubeIterator {
            cube: self,
            dp: 0.01,
            face: 0,
            point: (-self.edge_length / 2.0, -self.edge_length / 2.0),
        }
    }
}

pub struct CubeIterator<'a> {
    cube: &'a Cube,
    dp: f32,
    face: i32,
    point: (f32, f32),
}

impl<'a> Iterator for CubeIterator<'a> {
    type Item = (V3, V3);
    fn next(&mut self) -> Option<Self::Item> {
        let half_edge_length = self.cube.edge_length / 2.0;
        if self.point.1 <= half_edge_length {
            if self.point.0 <= half_edge_length {
                let vertex = match self.face {
                    0 => V3(self.point.0, self.point.1, -half_edge_length), // back
                    1 => V3(half_edge_length, self.point.1, self.point.0),  // right
                    2 => V3(self.point.0, half_edge_length, self.point.1),  // top
                    3 => V3(-half_edge_length, self.point.1, self.point.0), // left
                    4 => V3(self.point.0, self.point.1, half_edge_length),  // front
                    5 => V3(self.point.0, -half_edge_length, self.point.1), // bottom
                    _ => unreachable!(),
                };
                let norm = match self.face {
                    0 => V3(0.0, 0.0, -1.0),
                    1 => V3(1.0, 0.0, 0.0),
                    2 => V3(0.0, 1.0, 0.0),
                    3 => V3(-1.0, 0.0, 0.0),
                    4 => V3(0.0, 0.0, 1.0),
                    5 => V3(0.0, -1.0, 0.0),
                    _ => unreachable!(),
                };
                self.point.0 += self.dp;
                return Some((vertex + &self.cube.center, norm));
            } else {
                self.point.0 = -half_edge_length;
                self.point.1 += self.dp;
                return self.next();
            }
        } else {
            if self.face < 5 {
                self.face += 1;
                self.point = (-half_edge_length, -half_edge_length);
                return self.next();
            } else {
                return None;
            }
        }
    }
}
