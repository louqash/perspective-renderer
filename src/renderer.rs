use crate::common::*;
use crate::cube::Cube;
use crate::sphere::Sphere;

pub struct Context {
    screen: (usize, usize, usize),
    camera: (f32, f32),
    buffer: Vec<u8>,
    z_indexes: Vec<f32>,
}

impl Context {
    pub fn new(screen_w: usize, screen_h: usize) -> Context {
        Context {
            screen: (screen_w, screen_h, screen_w * screen_h),
            camera: (0.0, 0.0),
            buffer: vec![b' '; screen_w * screen_h],
            z_indexes: vec![std::f32::MIN; screen_w * screen_h],
        }
    }

    pub fn next_frame(&mut self) {
        self.buffer = vec![b' '; self.screen.2];
        self.z_indexes = vec![std::f32::MIN; self.screen.2];
    }

    pub fn get_frame(&self) -> &[u8] {
        &self.buffer
    }

    pub fn render_object<'a, Object: RenderObject<'a>>(&'a mut self, object: &'a Object)
    where
        Object::Iter: Iterator<Item = (V3, V3)>,
    {
        let light: (f32, f32, f32) = (1.0, 1.0, 1.0);
        let light_len = (light.0 * light.0 + light.1 * light.1 + light.2 * light.2).sqrt();

        let distance = 8.0;
        let scale = 10.0;

        let terminal_font_ratio = 10.0 / 21.0;
        for (vertex, norm) in object.cycle() {
            let (mut x, mut y, mut z) = (vertex.0, vertex.1, vertex.2);
            // let new_x = cos_phi * x + sin_phi * y;
            // let new_y = -sin_phi * x + cos_phi * y;
            // y = new_y;
            // x = new_x;

            // let new_y = cos_phi * y + sin_phi * z;
            // let new_z = -sin_phi * y + cos_phi * z;
            // y = new_y;
            // z = new_z;

            // let mut norm = (
            //     cos_phi * norm.0 + sin_phi * norm.1,
            //     cos_phi * norm.1 - sin_phi * norm.0,
            //     norm.2,
            // );
            // norm = (
            //     norm.0,
            //     cos_phi * norm.1 + sin_phi * norm.2,
            //     cos_phi * norm.2 - sin_phi * norm.1,
            // );

            let norm_len = (norm.0 * norm.0 + norm.1 * norm.1 + norm.2 * norm.2).sqrt();

            let dot_prod = norm.0 * light.0 + norm.1 * light.1 + norm.2 * light.2;
            let cos_angle = dot_prod / (norm_len * light_len);
            let ch = match (cos_angle * 6.0 + 6.0).ceil() as i32 {
                12 => b'@',
                11 => b'$',
                10 => b'J',
                9 => b'i',
                8 => b'*',
                7 => b'=',
                6 => b'~',
                5 => b'-',
                4 => b';',
                3 => b':',
                2 => b',',
                _ => b'.',
            };

            let pixel_dist_x = (x - self.camera.0) / distance * scale / terminal_font_ratio
                + (self.screen.0 as f32) / 2.0;
            let pixel_dist_y =
                -(y - self.camera.1) / distance * scale + (self.screen.1 as f32) / 2.0;
            let idx = (pixel_dist_y as usize) * self.screen.0 + pixel_dist_x as usize;
            if idx < self.screen.2 && self.z_indexes[idx] < z {
                self.buffer[idx] = ch;
                self.z_indexes[idx] = z;
            }
        }
    }
}
