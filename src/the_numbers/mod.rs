use opencv::core::{MatTraitConst, Vec3b};

use crate::{braille::calc_brightness, video::Renderer};

pub struct GodDamnItItsTheNumbers;

impl Renderer for GodDamnItItsTheNumbers {
    fn draw(&mut self, frame: &opencv::prelude::Mat) -> opencv::Result<Box<dyn crate::video::Renderable>> {
        let mut o = String::new();
        
        for x in 0..frame.rows() {
            for y in 0..frame.cols() {
                let pixel: &Vec3b = frame.at_2d(x, y)?;
                let how_much_are_my_eyes_burning = calc_brightness(*pixel);

                let num: char = if      how_much_are_my_eyes_burning >= 232 { '1' }
                                else if how_much_are_my_eyes_burning >= 216 { '2' }
                                else if how_much_are_my_eyes_burning >= 192 { '3' }
                                else if how_much_are_my_eyes_burning >= 144 { '4' }
                                else if how_much_are_my_eyes_burning >= 128 { '5' }
                                else if how_much_are_my_eyes_burning >= 80  { '6' }
                                else if how_much_are_my_eyes_burning >= 48  { '7' }
                                else if how_much_are_my_eyes_burning >= 32  { '8' }
                                else if how_much_are_my_eyes_burning >= 16  { '9' }
                                else                                        { '0' };

                o.push(num);
            }

            o.push('\n');
        }

        return Ok(Box::new(o))
    }
}