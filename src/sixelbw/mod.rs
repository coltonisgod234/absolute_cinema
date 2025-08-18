//! basic SIXEL implementation
//! 
//! uses a fixed threshold, unlike `sixelbw2` which dynamically decides it.
//! this is the basis for writing other sixel implementations.

use crate::sixel::*;
use opencv::{
    core::Vec3b,
    prelude::*,
};
use crate::video::{Renderable, Renderer};

pub struct SixelMono {
    pub threshold: u8
}

impl Renderer for SixelMono {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let mut output = String::new();
        output.push_str(BEGIN_SIXEL_BW);

        let height: i32 = frame.rows();
        let width: i32 = frame.cols();

        // what the fuck is happening anymore
        // for each vertical band of 6 pixels
        for y_start in (0..height).step_by(6) {
            for x in 0..width {
                let mut pixels: [bool; 6] = [false; 6];

                // collect pixels for this vertical band column
                for i in 0..6 {
                    let y = y_start + i;
                    if y < height {
                        let pixel_val: Vec3b = *frame.at_2d::<Vec3b>(y, x)?;
                        pixels[i as usize] = sixel_is_on_bw(pixel_val, self.threshold);
                    }
                }
                output.push(calculate_sixel_cols(pixels));
            }
            output.push('-');
        }

        output.push_str(END_SIXEL_BW);
        Ok(Box::new(output))
    }
}
