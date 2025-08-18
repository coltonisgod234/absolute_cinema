//! renders using foreground and background colours, for this to work right, ch is typically `▄`
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

pub struct HighRes {
    pub ch: char
}

impl Renderer for HighRes {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let mut frame_str = String::new();

        for row0 in (0..frame.rows()).step_by(2) {
            let row1 = row0 + 1;
            if row1 >= frame.rows() {
                break;
            }

            for col in 0..frame.cols() {
                let pixel0: Vec3b = *frame.at_2d(row0, col)?;
                let pixel1: Vec3b = *frame.at_2d(row1, col)?;

                let (rf, gf, bf) = (pixel0[2], pixel0[1], pixel0[0]);
                let (rg, gg, bg) = (pixel1[2], pixel1[1], pixel1[0]);
                write!(frame_str, "\x1B[48;2;{};{};{}m\x1B[38;2;{};{};{}m{}", rg, gg, bg, rf, gf, bf, self.ch).expect("write failed");
            }
            frame_str.push('\n');
        }
        if frame_str.ends_with('\n') {
            frame_str.pop(); // prevent vertical jitter
        }

        Ok(Box::new(frame_str))
    }
}

// ▀ ▄

/* unit testing
#[cfg(test)]
mod tests;
*/