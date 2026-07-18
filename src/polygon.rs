use crate::{framebuffer::Framebuffer, line::line};
use raylib::prelude::*;

pub fn draw_polygon(framebuffer: &mut Framebuffer, vertices: &[Vector2]) {
    if vertices.len() < 2 {
        return;
    }

    for index in 0..vertices.len() {
        let next = (index + 1) % vertices.len();
        line(framebuffer, vertices[index], vertices[next]);
    }
}
