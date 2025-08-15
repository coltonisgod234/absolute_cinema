use std::{
    fmt::Write as FmtWrite, io::{stdout, Write}
};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

pub fn render(frame: &Mat, term_width: u16, term_height: u16) -> opencv::Result<()> {
    let mut small_frame = Mat::default();
    resize(
        frame,
        &mut small_frame,
        opencv::core::Size { width: term_width as i32, height: term_height as i32 },
        0.0,
        0.0,
        INTER_LINEAR,
    )?;

    let mut frame_str = String::new();
    for row in 0..small_frame.rows() {
        for col in 0..small_frame.cols() {
            let pixel: Vec3b = *small_frame.at_2d(row, col)?;
            let (r, g, b) = (pixel[2], pixel[1], pixel[0]);
            write!(frame_str, "\x1B[48;2;{};{};{}m ", r, g, b).expect("write failed");
        }
        frame_str.push('\n');
    }
    if frame_str.ends_with('\n') {
        frame_str.pop(); // prevent vertical jitter
    }

    let mut stdout = stdout();
    print!("\x1B[H"); // cursor home
    write!(stdout, "{}", frame_str).expect("stdout write failed");
    stdout.flush().expect("stdout flush failed");

    Ok(())
}