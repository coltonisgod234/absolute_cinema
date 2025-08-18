use std::{
    fmt::Write as FmtWrite
};
use opencv::{
    core::Vec3b,
    prelude::*,
};
use crate::braille::{
    calc_brightness,
    find_codepoint,
    pixel_on
};
use crate::video::{Renderable, Renderer};

fn add_colours(a: &mut [u32; 3], b: Vec3b) {
    for i in 0..3 {
        a[i] += b[i] as u32;
    }
}

pub struct BrailleRGB {
    pub threshold: u8
}

impl Renderer for BrailleRGB {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let black: Vec3b = Vec3b::from([0,0,0]);

        let rows: i32 = frame.rows();
        let cols: i32 = frame.cols();
        let mut output = String::new();

        for row_start in (0..rows).step_by(2) {
            for col_start in (0..cols).step_by(4) {
                let mut colour_fg: [u32; 3] = [0,0,0];
                let mut colours_fg: u32 = 1u32;

                let mut colour_bg: [u32; 3] = [0,0,0];
                let mut colours_bg: u32 = 1u32;

                let mut pixels: [bool; 8] = [false;8];
                
                for dx in 0..4 {
                    for dy in 0..2 {
                        let pixel_y: i32 = row_start + dy;
                        let pixel_x: i32 = col_start + dx;
                        let pixel: Vec3b = if pixel_y < rows && pixel_x < cols {
                            *frame.at_2d::<Vec3b>(pixel_y, pixel_x)?
                        } else { black };

                        // calculate brightness of this pixel to determine if it's colour A or colour B
                        let brightness: u8 = calc_brightness(pixel);
                        let on: bool = pixel_on(brightness, self.threshold);
                        pixels[dy as usize * 2 + dx as usize] = on;

                        // true = colour A, false = colour B
                        if on {
                            add_colours(&mut colour_fg, pixel);
                            colours_fg += 1;
                        } else {
                            add_colours(&mut colour_bg, pixel);
                            colours_bg += 1;
                        }
                    }
                }
                // switch to the colours before drawing
                let ch: char = find_codepoint(pixels);
                write!(output, "\x1B[48;2;{};{};{}m\x1B[38;2;{};{};{}m{}", 
                    colour_bg[0] / colours_bg,
                    colour_bg[1] / colours_bg,
                    colour_bg[2] / colours_bg,
                    colour_fg[0] / colours_fg,
                    colour_fg[1] / colours_fg,
                    colour_fg[2] / colours_fg,
                    ch).expect("can't write to output str");
            }
            output.push('\n');
        }
        Ok(Box::new(output))
    }
}