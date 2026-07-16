use raylib::prelude::*;
use crate::line::line;
use crate::framebuffer::Framebuffer;

pub struct Polygon {
    pub vertices: Vec<Vector2>,
}

impl Polygon {
    pub fn new(vertices: Vec<Vector2>) -> Self {
        Polygon { vertices }
    }
    
    pub fn draw_outline(&self, framebuffer: &mut Framebuffer, color: Color) {
        framebuffer.set_current_color(color);
        let num_vertices = self.vertices.len();
        for i in 0..num_vertices {
            let start = self.vertices[i];
            let end = self.vertices[(i + 1) % num_vertices]; // Conecta el último con el primero
            line(framebuffer, start, end);
        }
    }

    pub fn fill(&self, framebuffer: &mut Framebuffer, color: Color) {
        if self.vertices.len() < 3 {
            return; 
        }

        framebuffer.set_current_color(color);

        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for v in &self.vertices {
            if v.y < min_y { min_y = v.y; }
            if v.y > max_y { max_y = v.y; }
        }

        let min_y = min_y.floor() as i32;
        let max_y = max_y.ceil() as i32;

        for y in min_y..=max_y {
            if y < 0 || y >= framebuffer.height as i32 {
                continue;
            }

            let mut intersections = Vec::new();
            let num_vertices = self.vertices.len();
            let y_f32 = y as f32;

            for i in 0..num_vertices {
                let v1 = self.vertices[i];
                let v2 = self.vertices[(i + 1) % num_vertices];

                let (p1, p2) = if v1.y < v2.y { (v1, v2) } else { (v2, v1) };

                if y_f32 >= p1.y && y_f32 < p2.y {
                    let x = p1.x + (y_f32 - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x);
                }
            }

            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            for pair in intersections.chunks(2) {
                if pair.len() == 2 {
                    let start_x = pair[0].ceil() as i32;
                    let end_x = pair[1].floor() as i32;

                    for x in start_x..=end_x {
                        if x >= 0 && x < framebuffer.width as i32 {
                            framebuffer.set_pixel(x as u32, y as u32);
                        }
                    }
                }
            }
        }
    }
}