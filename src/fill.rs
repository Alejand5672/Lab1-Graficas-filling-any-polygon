use crate::{framebuffer::Framebuffer, polygon::draw_polygon};
use raylib::prelude::*;

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    vertices: &[Vector2],
    border_color: Color,
    fill_color: Color,
) {
    fill_polygon_with_holes(framebuffer, vertices, &[], border_color, fill_color);
}

pub fn fill_polygon_with_holes(
    framebuffer: &mut Framebuffer,
    outer: &[Vector2],
    holes: &[&[Vector2]],
    border_color: Color,
    fill_color: Color,
) {
    if outer.len() < 3 {
        return;
    }

    framebuffer.set_current_color(border_color);
    draw_polygon(framebuffer, outer);
    for hole in holes {
        draw_polygon(framebuffer, hole);
    }

    framebuffer.set_current_color(fill_color);
    scanline_fill(framebuffer, outer, holes);

    framebuffer.set_current_color(border_color);
    draw_polygon(framebuffer, outer);
    for hole in holes {
        draw_polygon(framebuffer, hole);
    }
}

fn scanline_fill(framebuffer: &mut Framebuffer, outer: &[Vector2], holes: &[&[Vector2]]) {
    let mut min_y = outer.iter().map(|v| v.y as i32).min().unwrap();
    let mut max_y = outer.iter().map(|v| v.y as i32).max().unwrap();

    for contour in holes {
        if let Some(value) = contour.iter().map(|v| v.y as i32).min() {
            min_y = min_y.min(value);
        }
        if let Some(value) = contour.iter().map(|v| v.y as i32).max() {
            max_y = max_y.max(value);
        }
    }

    for y in min_y..=max_y {
        let scan_y = y as f32 + 0.5;
        let mut intersections = Vec::new();

        add_intersections(outer, scan_y, &mut intersections);
        for contour in holes {
            add_intersections(contour, scan_y, &mut intersections);
        }

        intersections.sort_by(|a, b| a.total_cmp(b));

        for pair in intersections.chunks_exact(2) {
            let start_x = pair[0].ceil() as i32;
            let end_x = pair[1].floor() as i32;

            for x in start_x..=end_x {
                framebuffer.point(x as usize, y as usize);
            }
        }
    }
}

fn add_intersections(vertices: &[Vector2], scan_y: f32, intersections: &mut Vec<f32>) {
    if vertices.len() < 3 {
        return;
    }

    for index in 0..vertices.len() {
        let first = vertices[index];
        let second = vertices[(index + 1) % vertices.len()];
        let crosses_scanline =
            (first.y <= scan_y && second.y > scan_y) || (second.y <= scan_y && first.y > scan_y);

        if crosses_scanline {
            let x = first.x + (scan_y - first.y) * (second.x - first.x) / (second.y - first.y);
            intersections.push(x);
        }
    }
}
