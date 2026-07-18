use image::{Rgb, RgbImage};
use raylib::prelude::*;

pub struct Framebuffer {
    width: u32,
    height: u32,
    image: RgbImage,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            image: RgbImage::new(width, height),
            background_color: Color::BLACK,
            current_color: Color::WHITE,
        }
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn clear(&mut self) {
        let color = Rgb([
            self.background_color.r,
            self.background_color.g,
            self.background_color.b,
        ]);

        for y in 0..self.height {
            for x in 0..self.width {
                self.image.put_pixel(x, y, color);
            }
        }
    }

    pub fn point(&mut self, x: usize, y: usize) {
        if x < self.width as usize && y < self.height as usize {
            self.image.put_pixel(
                x as u32,
                y as u32,
                Rgb([
                    self.current_color.r,
                    self.current_color.g,
                    self.current_color.b,
                ]),
            );
        }
    }

    pub fn render_to_file(&self, filename: &str) {
        self.image.save(filename).unwrap();
    }
}
