//! a better sixel algorithm
//! 
//! calculates the average brightness of the frame, adjusts it by
//! `adjust`, then compares against that value to determine if the sixel is on
//! or off.
//! 
//! black-and-white only
//! 
//! requires `sixelbw`

use rayon::prelude::*;
use opencv::{
    core::Vec3b,
    prelude::*,
};
use crate::{sixel::{
    calculate_sixel_cols, sixel_is_on_bw, BEGIN_SIXEL_BW, END_SIXEL_BW
}, video::{Renderable, Renderer}};

/// calculate the average brightness of a frame
pub fn calc_avg_brightness(frame: &Mat, width: i32, height: i32, adjust: i8) -> opencv::Result<i32> {
    // calculate the average brightness
    let total_px: i32 = width * height;
    let mut avg_brightness_acc: i32 = 0;

    for x in 0..width {
        for y in 0..height {
            let pixel: Vec3b = *frame.at_2d::<Vec3b>(y, x)?;  // BGR
            let brightness: u8 = (
                0.299 * pixel[2] as f32
                + 0.587 * pixel[1] as f32
                + 0.114 * pixel[0] as f32) as u8;

            avg_brightness_acc += brightness as i32;
        }
    }
    avg_brightness_acc /= total_px as i32;
    avg_brightness_acc += adjust as i32;
    return Ok(avg_brightness_acc);
}

pub struct SixelMono2 {
    pub adjust: i8
}

impl Renderer for SixelMono2 {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let height: i32 = frame.rows();
        let width: i32 = frame.cols();
        let average_brightness: i32 = calc_avg_brightness(&frame, width, height, self.adjust)?;

        let slices: Vec<_> = (0..height).step_by(6).collect();

        // for each vertical band of 6 pixels
        let chunks: Vec<String> = slices.par_iter().map(|&y_start| {
            let mut chunk = String::new();
            for x in 0..width {
                let mut pixels: [bool; 6] = [false; 6];

                // collect pixels for this vertical band column
                for i in 0..6 {
                    let y = y_start + i;
                    if y < height {
                        let pixel_val: Vec3b = *frame.at_2d::<Vec3b>(y, x)
                            .expect("failed to get pixel colour");

                        pixels[i as usize] = sixel_is_on_bw(pixel_val, average_brightness as u8);
                    }
                }
                chunk.push(calculate_sixel_cols(pixels));
            }
            return chunk
        }).collect();

        let mut text = String::new();

        text.push_str(BEGIN_SIXEL_BW);
        text.push_str(&chunks.join("-"));
        text.push_str(END_SIXEL_BW);

        Ok(Box::new(text))
    }
}
