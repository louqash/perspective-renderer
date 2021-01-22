use crate::common::*;
use crate::renderer::RenderObject;

#[derive(Clone, Debug)]
pub struct Cube {
    center: V3,
    edge_length: f32,
    vertices: [V3; 8],
    normal_vectors: [V3; 6],
}

impl Cube {
    pub fn new(center: V3, edge_length: f32) -> Cube {
        let halfsize = edge_length/2.0;
        let vertices: [V3; 8] = [
            V3( halfsize,  halfsize,  halfsize),
            V3( halfsize, -halfsize,  halfsize),
            V3(-halfsize, -halfsize,  halfsize),
            V3(-halfsize,  halfsize,  halfsize),
            V3( halfsize,  halfsize, -halfsize),
            V3( halfsize, -halfsize, -halfsize),
            V3(-halfsize, -halfsize, -halfsize),
            V3(-halfsize,  halfsize, -halfsize),
        ];
        let normal_vectors = [
            V3(0.0, 0.0, 1.0),
            V3(1.0, 0.0, 0.0),
            V3(0.0, 0.0, -1.0),
            V3(-1.0, 0.0, 0.0),
            V3(0.0, 1.0, 0.0),
            V3(0.0, -1.0, 0.0),
        ];
        Cube {
            center,
            edge_length,
            vertices,
            normal_vectors
        }
    }
    pub fn rotate_x(&mut self, phi: f32) {
        for vertex in self.vertices.iter_mut() {
            let new_y = phi.cos() * vertex.1 + phi.sin() * vertex.2;
            let new_z = -phi.sin() * vertex.1 + phi.cos() * vertex.2;
            vertex.1 = new_y;
            vertex.2 = new_z;
        }
        for vector in self.normal_vectors.iter_mut() {
            let new_y = phi.cos() * vector.1 + phi.sin() * vector.2;
            let new_z = -phi.sin() * vector.1 + phi.cos() * vector.2;
            vector.1 = new_y;
            vector.2 = new_z;
        }
    }
    pub fn rotate_y(&mut self, phi: f32) {
        for vertex in self.vertices.iter_mut() {
            let new_x = phi.cos() * vertex.0 - phi.sin() * vertex.2;
            let new_z = phi.sin() * vertex.0 + phi.cos() * vertex.2;
            vertex.0 = new_x;
            vertex.2 = new_z;
        }
        for vector in self.normal_vectors.iter_mut() {
            let new_x = phi.cos() * vector.0 - phi.sin() * vector.2;
            let new_z = phi.sin() * vector.0 + phi.cos() * vector.2;
            vector.0 = new_x;
            vector.2 = new_z;
        }
    }
    pub fn rotate_z(&mut self, phi: f32) {
        for vertex in self.vertices.iter_mut() {
            let new_x = phi.cos() * vertex.0 + phi.sin() * vertex.1;
            let new_y = -phi.sin() * vertex.0 + phi.cos() * vertex.1;
            vertex.0 = new_x;
            vertex.1 = new_y;
        }
        for vector in self.normal_vectors.iter_mut() {
            let new_x = phi.cos() * vector.0 + phi.sin() * vector.1;
            let new_y = -phi.sin() * vector.0 + phi.cos() * vector.1;
            vector.0 = new_x;
            vector.1 = new_y;
        }
    }
}
impl<'a> RenderObject<'a> for Cube {
    type Iter = CubeIterator<'a>;
    fn cycle(&'a self) -> Self::Iter {
        CubeIterator {
            cube: self,
            face: 0,
            steps: (0, 0),
            step_count: 100
        }
    }
    fn position(&'a self) -> V3 {
        self.center
    }
}

pub struct CubeIterator<'a> {
    cube: &'a Cube,
    face: i32,
    steps: (usize, usize),
    step_count: usize,
}

impl<'a> Iterator for CubeIterator<'a> {
    type Item = (V3, V3);
    fn next(&mut self) -> Option<Self::Item> {
        let (start, (end_0, end_1)) = match self.face {
            0 => { // front
                (&self.cube.vertices[2],
                (&self.cube.vertices[1], &self.cube.vertices[3]))
            },
            1 => { // right
                (&self.cube.vertices[5],
                (&self.cube.vertices[1], &self.cube.vertices[4]))
            },
            2 => { // back
                (&self.cube.vertices[6],
                (&self.cube.vertices[5], &self.cube.vertices[7]))
            },
            3 => { // left
                (&self.cube.vertices[6],
                (&self.cube.vertices[2], &self.cube.vertices[7]))
            },
            4 => { // top
                (&self.cube.vertices[7],
                (&self.cube.vertices[4], &self.cube.vertices[3]))
            },
            5 => { // bottom
                (&self.cube.vertices[6],
                (&self.cube.vertices[5], &self.cube.vertices[2]))
            },
            _ => unreachable!(),
        };
        if self.steps.1 < self.step_count {
            if self.steps.0 < self.step_count {
                let d0 = (end_0 - start) / self.step_count as f32;
                let d1 = (end_1 - start) / self.step_count as f32;
                let point = start + d0*( self.steps.0 as f32 ) + d1*( self.steps.1 as f32 );
                self.steps.0 += 1;
                return Some((point + &self.cube.center, self.cube.normal_vectors[self.face as usize]));
            } else {
                self.steps.0 = 0;
                self.steps.1 += 1;
                return self.next();
            }
        } else {
            if self.face < 5 {
                self.face += 1;
                self.steps = (0, 0);
                return self.next();
            } else {
                return None;
            }
        }
    }
}
