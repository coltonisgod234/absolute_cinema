use opencv::{
    core::Vec3b,
    prelude::*,
};
use rayon::{
    prelude::*
};
use std::fmt::Write;
use crate::video::*;
use crate::sixel::*;

pub struct SixelMultipass {
    pub colours: Vec<(u8,u8,u8)>,
    start_string: String
}

impl SixelMultipass {
    pub fn new(colours: Vec<(u8,u8,u8)>) -> Self {
        let renderer: SixelMultipass = Self {
            start_string: determine_start_string(&colours),
            colours: colours
        };
        return renderer
    }

    fn start_sixel(&self, output: &mut String) {
        output.push_str(BEGIN_SIXEL_BW);
        output.push_str(&self.start_string);
    }

    fn stop_sixel(&self, output: &mut String) {
        output.push_str(END_SIXEL_BW);
    }
}

impl Renderer for SixelMultipass {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let mut output = String::new();

        let height: i32 = frame.rows();
        let width: i32 = frame.cols();

        // preform a pass
        for (idx, _) in self.colours.iter().enumerate() {
            write!(output, "#{}", idx).unwrap();  // switch to this ch
            print!("\x1B[H");  // go home
            self.start_sixel(&mut output);


            for y_start in (0..height).step_by(6) {
                for x in 0..width {
                    let mut pixels: [bool; 6] = [false; 6];

                    // collect pixels for this vertical band column
                    for dy in 0..6 {
                        let y = y_start + dy;
                        if y < height {
                            //let pixel: Vec3b = *frame.at_2d::<Vec3b>(y, x)?;
                            let pixel = get_px(&frame, x, y);

                            // check if the pixel is the same as the one we're targetting
                            let on: bool = closest_colour_index((pixel[2], pixel[1], pixel[0]), &self.colours).unwrap() == idx;
                            pixels[dy as usize] = on;
                        }
                    }
                    output.push(calculate_sixel_cols(pixels));
                }
                output.push('-');
            }

            self.stop_sixel(&mut output);
        }

        Ok(Box::new(output))
    }
}