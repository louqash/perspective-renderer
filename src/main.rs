mod common;
mod cube;
mod renderer;

use std::io::Write;
use std::{thread, time};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    let screen_size = terminal_size()
        .map(|(Width(w), Height(h))| (w as usize, h as usize))
        .unwrap();
    let context = renderer::Context::new(screen_size.0, screen_size.1);

    let mut phi: f32 = 0.0;
    loop {
        let _ = std::io::stdout().write_all(&context.render_cube(phi)[..]);
        thread::sleep(time::Duration::from_millis(50));
        phi += 0.1;
    }
}
