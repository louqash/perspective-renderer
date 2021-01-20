mod common;
mod cube;
mod renderer;
mod sphere;

use std::io::Write;
use std::{thread, time};
use terminal_size::{terminal_size, Height, Width};
use common::*;
use sphere::Sphere;

fn main() {
    let screen_size = terminal_size()
        .map(|(Width(w), Height(h))| (w as usize, h as usize))
        .unwrap();
    let mut context = renderer::Context::new(screen_size.0, screen_size.1);
    let sphere = Sphere::new(V3(0.0, 0.0, 0.0), 12.0);

    let mut phi: f32 = 0.0;
    loop {
        context.render_object(&sphere);
        let _ = std::io::stdout().write_all(context.get_frame());
        context.next_frame();
        thread::sleep(time::Duration::from_millis(50));
        phi += 0.1;
    }
}
