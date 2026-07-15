use raylib::prelude::*;
use crate::line::line;
use crate::framebuffer::Framebuffer;

pub fn polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vector2],
) {
    let num_vertices = vertices.len();
    for i in 0..num_vertices {
        let start = vertices[i];
        let end = vertices[(i + 1) % num_vertices]; 
        line(framebuffer, start, end);
    }
}   



