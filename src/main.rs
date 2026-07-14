mod framebuffer;
mod line;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600, Color::BLACK);
    framebuffer.clear();
    line(&mut framebuffer, Vector2::new(100.0, 100.0), Vector2::new(200.0, 200.0));
    framebuffer.render_to_file("output.png");
}
