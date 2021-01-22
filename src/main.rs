mod common;
mod cube;
mod renderer;
mod sphere;

use std::io::Write;
use std::{thread, time};
use terminal_size::{terminal_size, Height, Width};
use common::*;
use sphere::Sphere;
use cube::Cube;

fn main() {
    let screen_size = terminal_size()
        .map(|(Width(w), Height(h))| (w as usize, h as usize))
        .unwrap();
    let mut context = renderer::Context::new(screen_size.0, screen_size.1);
    let mut cube = Cube::new(V3(0.0 , 0.0 , 10.0), 6.0);
    let sphere = Sphere::new(V3(30.0, 10.0, 20.0), 12.0);

    let phi = (0.1, 0.2, -0.3);
    loop {
        context.render_object(&cube);
        context.render_object(&sphere);
        cube.rotate_x(phi.0);
        cube.rotate_y(phi.1);
        cube.rotate_z(phi.2);
        let _ = std::io::stdout().write_all(context.get_frame());
        context.next_frame();
        thread::sleep(time::Duration::from_millis(100));
    }
}
