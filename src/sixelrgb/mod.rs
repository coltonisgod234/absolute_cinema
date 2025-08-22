use opencv::{
    core::Vec3b,
    prelude::*,
};
use std::fmt::Write;
use crate::video::*;
use crate::sixel::*;
use crate::sixelbw2::calc_avg_brightness;

pub struct SixelColour {
    pub colours: Vec<(u8,u8,u8)>,
    pub adjust: i8,
    pub alpha: f32,
    start_string: String
}

impl SixelColour {
    pub fn new(colours: Vec<(u8,u8,u8)>, adjust: i8, alpha: f32) -> Self {
        let renderer: SixelColour = Self {
            start_string: determine_start_string(&colours),
            adjust: adjust,
            alpha: alpha,
            colours: colours
        };
        return renderer
    }
}

impl Renderer for SixelColour {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let mut output = String::new();
        output.push_str(BEGIN_SIXEL_BW);
        output.push_str(&self.start_string);

        let height: i32 = frame.rows();
        let width: i32 = frame.cols();
        let avg_brightness: u8 = calc_avg_brightness(
            &frame,
            width,
            height,
            self.adjust,
            self.alpha,
            0u8)?;

        for y_start in (0..height).step_by(6) {
            for x in 0..width {
                let mut avg_colour = (0,0,0);
                let mut pixels: [bool; 6] = [false; 6];

                // collect pixels for this vertical band column
                for i in 0..6 {
                    let y = y_start + i;
                    if y < height {
                        let pixel_val: Vec3b = *frame.at_2d::<Vec3b>(y, x)?;
                        let on: bool = sixel_is_on_bw(pixel_val, avg_brightness);
                        pixels[i as usize] = on;

                        // weird backwords because BGR is amazing
                        if on {
                            avg_colour.2 += pixel_val[2] as i32;
                            avg_colour.1 += pixel_val[1] as i32;
                            avg_colour.0 += pixel_val[0] as i32;
                        }
                    }
                }
                avg_colour.2 /= 6;
                avg_colour.1 /= 6;
                avg_colour.0 /= 6;  // guestimate, disgusting but I don't care to keep count

                write!(output,
                    "#{}",
                    closest_colour_index(
                        (avg_colour.0 as u8, avg_colour.1 as u8, avg_colour.2 as u8),
                        &self.colours
                    ).unwrap()
                ).expect("failed to write to output");

                output.push(calculate_sixel_cols(pixels));
            }
            output.push('-');
        }

        output.push_str(END_SIXEL_BW);
        Ok(Box::new(output))
    }
}