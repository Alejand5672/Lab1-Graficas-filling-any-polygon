mod fill;
mod framebuffer;
mod line;
mod polygon;

use fill::{fill_polygon, fill_polygon_with_holes};
use framebuffer::Framebuffer;
use raylib::prelude::*;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);
    framebuffer.set_background_color(Color::WHITE);
    framebuffer.clear();

    let polygon1 = [
        Vector2::new(165.0, 380.0),
        Vector2::new(185.0, 360.0),
        Vector2::new(180.0, 330.0),
        Vector2::new(207.0, 345.0),
        Vector2::new(233.0, 330.0),
        Vector2::new(230.0, 360.0),
        Vector2::new(250.0, 380.0),
        Vector2::new(220.0, 385.0),
        Vector2::new(205.0, 410.0),
        Vector2::new(193.0, 383.0),
    ];
    let polygon2 = [
        Vector2::new(321.0, 335.0),
        Vector2::new(288.0, 286.0),
        Vector2::new(339.0, 251.0),
        Vector2::new(374.0, 302.0),
    ];
    let polygon3 = [
        Vector2::new(377.0, 249.0),
        Vector2::new(411.0, 197.0),
        Vector2::new(436.0, 249.0),
    ];
    let polygon4 = [
        Vector2::new(413.0, 177.0),
        Vector2::new(448.0, 159.0),
        Vector2::new(502.0, 88.0),
        Vector2::new(553.0, 53.0),
        Vector2::new(535.0, 36.0),
        Vector2::new(676.0, 37.0),
        Vector2::new(660.0, 52.0),
        Vector2::new(750.0, 145.0),
        Vector2::new(761.0, 179.0),
        Vector2::new(672.0, 192.0),
        Vector2::new(659.0, 214.0),
        Vector2::new(615.0, 214.0),
        Vector2::new(632.0, 230.0),
        Vector2::new(580.0, 230.0),
        Vector2::new(597.0, 215.0),
        Vector2::new(552.0, 214.0),
        Vector2::new(517.0, 144.0),
        Vector2::new(466.0, 180.0),
    ];
    let hole = [
        Vector2::new(682.0, 175.0),
        Vector2::new(708.0, 120.0),
        Vector2::new(735.0, 148.0),
        Vector2::new(739.0, 170.0),
    ];

    fill_polygon(&mut framebuffer, &polygon1, Color::BLACK, Color::YELLOW);
    fill_polygon(&mut framebuffer, &polygon2, Color::BLACK, Color::BLUE);
    fill_polygon(&mut framebuffer, &polygon3, Color::BLACK, Color::RED);
    fill_polygon_with_holes(
        &mut framebuffer,
        &polygon4,
        &[&hole],
        Color::BLACK,
        Color::GREEN,
    );

    framebuffer.render_to_file("out.png");
}
