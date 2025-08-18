//! renders using the ` ` (space) character and background colours
//! 
//! supports true-colour

use std::{
    fmt::Write as FmtWrite
};
use opencv::{
    core::Vec3b,
    prelude::*,
};
use crate::video::{Renderable, Renderer};

pub struct LowRes {
    pub ch: char
}

impl Renderer for LowRes {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let mut frame_str = String::new();

        for row in 0..frame.rows() {
            for col in 0..frame.cols() {
                let pixel: Vec3b = *frame.at_2d(row, col)?;
                let (red, green, blue) = (pixel[2], pixel[1], pixel[0]);
                write!(frame_str, "\x1B[48;2;{};{};{}m{}",
                    red,
                    green,
                    blue,
                    self.ch).expect("write failed");
            }
            frame_str.push('\n');
        }
        if frame_str.ends_with('\n') {
            frame_str.pop(); // prevent vertical jitter
        }

        Ok(Box::new(frame_str))
    }
}
