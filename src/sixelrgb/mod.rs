use opencv::{
    core::Vec3b,
    prelude::*,
};
use std::fmt::Write;
use crate::video::*;
use crate::sixel::*;
use crate::sixelbw2::calc_avg_brightness;

pub struct SixelGreyscale {
    pub colours: Vec<(u8,u8,u8)>,
    pub adjust: i8,
    pub alpha: f32,
    start_string: String
}

fn determine_start_string(colours: &Vec<(u8,u8,u8)>) -> String {
    let mut output = String::new();
    for (idx, colour) in colours.iter().enumerate() {
        let string: &str = &format!("#{};2;{};{};{}",
            idx,
            rgb_to_sixel_percent(colour.0),  // R
            rgb_to_sixel_percent(colour.1),  // G
            rgb_to_sixel_percent(colour.2)
        );
        output.push_str(string);
    }
    return output
}

fn closest_colour_index(target: (u8, u8, u8), colours: &[(u8, u8, u8)]) -> Option<usize> {
    colours.iter().enumerate().min_by_key(|&(_, &(r, g, b))| {
        let dr = r as i32 - target.0 as i32;
        let dg = g as i32 - target.1 as i32;
        let db = b as i32 - target.2 as i32;
        dr * dr + dg * dg + db * db
    }).map(|(i, _)| i)
}


impl SixelGreyscale {
    pub fn new(colours: Vec<(u8,u8,u8)>, adjust: i8, alpha: f32) -> Self {
        let renderer: SixelGreyscale = Self {
            start_string: determine_start_string(&colours),
            adjust: adjust,
            alpha: alpha,
            colours: colours
        };
        return renderer
    }
}

impl Renderer for SixelGreyscale {
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
                let mut avg_colour: (i32, i32, i32) = (0i32,0i32,0i32);
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
                            avg_colour.0 += pixel_val[2] as i32;
                            avg_colour.1 += pixel_val[1] as i32;
                            avg_colour.2 += pixel_val[0] as i32;
                        }
                    }
                }
                avg_colour.0 /= 6;  // guestimate, disgusting but I don't care to keep count
                avg_colour.1 /= 6;
                avg_colour.2 /= 6;

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