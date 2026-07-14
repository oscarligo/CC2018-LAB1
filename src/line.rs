use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

// bresenham's line algorithm


pub fn line (
    framebuffer: &mut Framebuffer,
    start: Vector2,
    end: Vector2,
) {

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;        
    let x1 = end.x as i32;
    let y1 = end.y as i32;  

    let dx = (x1 - x0).abs(); 
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };  

    let mut err = dx + dy;  /* error value e_xy */

    loop {
        framebuffer.set_pixel(x0 as u32, y0 as u32, framebuffer.current_color);

        if x0 == x1 && y0 == y1 { break; }

        let e2 = 2 * err;
        if e2 >= dy { 
            err += dy; 
            x0 += sx; 
        } /* e_xy+e_x > 0 */
        if e2 <= dx { 
            err += dx; 
            y0 += sy; 
        } /* e_xy+e_y < 0 */
    }





}