use crate::common::*;

pub trait RenderObject<'a> {
    type Iter: Iterator<Item = (V3, V3)>;
    fn cycle(&'a self) -> Self::Iter;
}
pub struct Context {
    screen: (usize, usize),
    pixel_count: usize,
    camera: (f32, f32),
    buffer: Vec<u8>,
    z_indexes: Vec<f32>,
}

impl Context {
    pub fn new(screen_w: usize, screen_h: usize) -> Context {
        Context {
            screen: (screen_w, screen_h),
            pixel_count: screen_w * screen_h,
            camera: (0.0, 0.0),
            buffer: vec![b' '; screen_w * screen_h],
            z_indexes: vec![std::f32::MIN; screen_w * screen_h],
        }
    }

    pub fn next_frame(&mut self) {
        self.buffer = vec![b' '; self.pixel_count];
        self.z_indexes = vec![std::f32::MIN; self.pixel_count];
    }

    pub fn get_frame(&self) -> &[u8] {
        &self.buffer
    }

    pub fn render_object<'a, Object: RenderObject<'a>>(&'a mut self, object: &'a Object) {
        let light = V3(1.0, 1.0, 1.0);

        let distance = 8.0;
        let scale = 10.0;

        let terminal_font_ratio = 10.0 / 21.0;
        for (vertex, norm) in object.cycle() {
            let V3(x, y, z) = vertex;
            let norm_len = norm.len();

            let dot_prod = norm.dot(&light);
            let cos_angle = dot_prod / (norm_len * light.len());
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
            if idx < self.pixel_count && self.z_indexes[idx] < z {
                self.buffer[idx] = ch;
                self.z_indexes[idx] = z;
            }
        }
    }
}
