use std::ops::Add;
use std::io::Write;
use std::{thread, time};
use terminal_size::{Width, Height, terminal_size};

#[derive(Clone, Copy, Debug)]
struct V3 (f32, f32, f32);
impl Add<V3> for V3 {
    type Output = V3;
    fn add(self, other: V3) -> V3 {
        V3(self.0.add(other.0),
           self.1.add(other.1),
           self.2.add(other.2))

    }
}

#[derive(Debug)]
struct Object {
    vertices: Vec<V3>,
    faces: Vec<usize>,
    face_order: usize
}

fn get_cube(position: V3, size: f32) -> Object {
    let mut vertices = Vec::with_capacity(8);
    let halfsize = size/2.0;
    vertices.push(position + V3( halfsize,  halfsize,  halfsize));
    vertices.push(position + V3( halfsize,  halfsize, -halfsize));
    vertices.push(position + V3( halfsize, -halfsize,  halfsize));
    vertices.push(position + V3( halfsize, -halfsize, -halfsize));
    vertices.push(position + V3(-halfsize,  halfsize,  halfsize));
    vertices.push(position + V3(-halfsize,  halfsize, -halfsize));
    vertices.push(position + V3(-halfsize, -halfsize,  halfsize));
    vertices.push(position + V3(-halfsize, -halfsize, -halfsize));

    let faces = vec![0, 1, 3, 2,
                     0, 1, 5, 4,
                     0, 2, 6, 4,
                     7, 3, 2, 6,
                     7, 5, 4, 6,
                     7, 3, 1, 5];

    let cube = Object {
        vertices: vertices,
        faces: faces,
        face_order: 4
    };

    cube
}

fn main() {
    // let cube = get_cube(V3(0.0, 0.0, 0.0), 16.0);
    // let camera = V3(0.0, 0.0, 0.0);
    let screen_size = terminal_size().map(|(Width(w), Height(h))| (w as usize, h as usize)).unwrap();
    let pixel_count = screen_size.0 * screen_size.1;
    let light: (f32, f32, f32) = (-1.0, -1.0, -1.0);
    let light_len = (light.0*light.0 + light.1*light.1 + light.2*light.2).sqrt();

    let cube_edge_length = 6.0;
    let half_edge_length = cube_edge_length as f32/2.0;
    let distance = 8.0;
    let inv_cube_distance = 1.0/distance;
    let scale = 10.0;
    let dx = 0.01;
    let dy = 0.01;

    let mut phi: f32 = 0.0;
    loop {
        let mut buf = vec![b' '; pixel_count as usize];
        let mut z_mem = vec![std::f32::MIN; pixel_count as usize];
        let mut y = 0.0;
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        while y <= cube_edge_length {
            let mut x = 0.0;
            while x <= cube_edge_length {
                for face in 0..6 {
                    let x = x - half_edge_length;
                    let y = y - half_edge_length;
                    let (mut x, mut y, mut z) = match face {
                        0 => (x, y, -half_edge_length),
                        1 => (half_edge_length, y, x),
                        2 => (x, half_edge_length, y),
                        3 => (-half_edge_length, y, x),
                        4 => (x, y, half_edge_length),
                        5 => (x, -half_edge_length, y),
                        _ => unreachable!()
                    };
                    let norm = match face {
                        0 => (0.0, 0.0, 1.0),
                        1 => (1.0, 0.0, 0.0),
                        2 => (0.0, 1.0, 0.0),
                        3 => (-1.0, 0.0, 0.0),
                        4 => (0.0, 0.0, -1.0),
                        5 => (0.0, -1.0, 0.0),
                        _ => unreachable!()
                    };

                    let new_x = cos_phi*x + sin_phi*y;
                    let new_y = -sin_phi*x + cos_phi*y;
                    y = new_y;
                    x = new_x;

                    let new_y = cos_phi*y + sin_phi*z;
                    let new_z = -sin_phi*y + cos_phi*z;
                    y = new_y;
                    z = new_z;

                    let mut norm = (
                        cos_phi*norm.0 + sin_phi*norm.1,
                        cos_phi*norm.1 - sin_phi*norm.0,
                        norm.2);
                    norm = (
                        norm.0,
                        cos_phi*norm.1 + sin_phi*norm.2,
                        cos_phi*norm.2 - sin_phi*norm.1
                    );

                    let norm_len = (norm.0*norm.0 + norm.1*norm.1 + norm.2*norm.2).sqrt();

                    let dot_prod = norm.0*light.0 + norm.1*light.1 + norm.2*light.2;
                    let cos_angle = dot_prod / (norm_len * light_len);
                    let ch = match (cos_angle.abs() * 12.0).ceil() as i32 {
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
                        _ => b'.'
                    };

                    let pixel_dist_x = x * inv_cube_distance * scale + (screen_size.0 as f32)/2.0;
                    let pixel_dist_y = y * inv_cube_distance * scale + (screen_size.1 as f32)/2.0;
                    let idx = (pixel_dist_y as usize) * screen_size.0 + pixel_dist_x as usize;
                    if z_mem[idx] < z && idx < pixel_count {
                        buf[idx] = ch;
                        z_mem[idx] = z;
                    }

                }
                x += dx;
            }
            y += dy;
        }
        phi += 0.1;
        let _ = std::io::stdout().write_all(&buf[..]);
        thread::sleep(time::Duration::from_millis(100));

    }
}
