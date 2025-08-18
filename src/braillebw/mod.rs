use opencv::{
    core::{Vec3b, VecN},
    prelude::*
};
use crate::video::{Renderer, Renderable};
use crate::braille::{calc_brightness, find_codepoint, pixel_on};

pub struct Braillebw {
    pub threshold: u8
}

impl Renderer for Braillebw {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        const BLACK: Vec3b = VecN([0,0,0]);

        let rows: i32 = frame.rows();
        let cols: i32 = frame.cols();

        let mut output = String::new();

        for row_start in (0..rows).step_by(2) {
            for col_start in (0..cols).step_by(4) {
                let mut pixels: [bool; 8] = [false;8];
                for dx in 0..4 {
                    for dy in 0..2 {
                        let pixel_y: i32 = row_start + dy;
                        let pixel_x: i32 = col_start + dx;
                        let pixel: Vec3b = if pixel_y < rows && pixel_x < cols {
                            *frame.at_2d::<Vec3b>(pixel_y, pixel_x)?
                        } else { BLACK };

                        let brightness: u8 = calc_brightness(pixel);
                        pixels[dy as usize * 2 + dx as usize] = pixel_on(brightness, self.threshold);
                    }
                }
                let ch: char = find_codepoint(pixels);
                output.push(ch);
            }
            output.push('\n');
        }
        return Ok(Box::new(output))
    }
}
