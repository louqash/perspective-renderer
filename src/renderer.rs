use crate::common::*;
use crate::cube::Cube;
use crate::sphere::Sphere;

pub struct Context {
    screen: (usize, usize),
    camera: (f32, f32),
}

impl Context {
    pub fn new(screen_w: usize, screen_h: usize) -> Context {
        Context {
            screen: (screen_w, screen_h),
            camera: (0.0, 0.0),
        }
    }

    pub fn render_cube(&self, phi: f32) -> Vec<u8> {
        let pixel_count = self.screen.0 * self.screen.1;
        let light: (f32, f32, f32) = (1.0, 1.0, 1.0);
        let light_len = (light.0 * light.0 + light.1 * light.1 + light.2 * light.2).sqrt();

        let distance = 8.0;
        let scale = 10.0;

        let mut buf = vec![b' '; pixel_count as usize];
        let mut z_mem = vec![std::f32::MIN; pixel_count as usize];
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let terminal_font_ratio = 10.0/21.0;
        // let cube = Cube::new(V3(0.0, 0.0, 0.0), 6.0);
        let sphere = Sphere::new(V3(0.0, 0.0, 0.0), 6.0);
        for (vertex, norm) in sphere.into_iter() {
            let (mut x, mut y, mut z) = (vertex.0, vertex.1, vertex.2);
            let new_x = cos_phi * x + sin_phi * y;
            let new_y = -sin_phi * x + cos_phi * y;
            y = new_y;
            x = new_x;

            let new_y = cos_phi * y + sin_phi * z;
            let new_z = -sin_phi * y + cos_phi * z;
            y = new_y;
            z = new_z;

            let mut norm = (
                cos_phi * norm.0 + sin_phi * norm.1,
                cos_phi * norm.1 - sin_phi * norm.0,
                norm.2,
            );
            norm = (
                norm.0,
                cos_phi * norm.1 + sin_phi * norm.2,
                cos_phi * norm.2 - sin_phi * norm.1,
            );

            let norm_len = (norm.0 * norm.0 + norm.1 * norm.1 + norm.2 * norm.2).sqrt();

            let dot_prod = norm.0 * light.0 + norm.1 * light.1 + norm.2 * light.2;
            let cos_angle = dot_prod / (norm_len * light_len);
            let ch = match (cos_angle * 6.0 + 6.0).ceil() as i32 {
                12 => b'@',
                11 => b'$',
                10 => b'#',
                9 => b'*',
                8 => b'!',
                7 => b'=',
                6 => b';',
                5 => b':',
                4 => b'~',
                3 => b'-',
                2 => b',',
                _ => b'.',
            };

            let pixel_dist_x =
                (x - self.camera.0) / distance * scale / terminal_font_ratio + (self.screen.0 as f32) / 2.0;
            let pixel_dist_y =
                -(y - self.camera.1) / distance * scale + (self.screen.1 as f32) / 2.0;
            let idx = (pixel_dist_y as usize) * self.screen.0 + pixel_dist_x as usize;
            if idx < pixel_count && z_mem[idx] < z {
                buf[idx] = ch;
                z_mem[idx] = z;
            }
        }
        buf
    }
}
