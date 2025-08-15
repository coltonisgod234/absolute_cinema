use std::{
    fmt::Write as FmtWrite, io::{stdout, Write}
};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

/// renders using foreground and background colours, for this to work right, ch is typically `▄`
/// 
/// supports true-colour
pub fn render_frame_hi_res(frame: &Mat, term_width: u16, term_height: u16, ch: char) -> opencv::Result<()> {
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
    for row0 in (0..small_frame.rows()).step_by(2) {
        let row1 = row0 + 1;
        if row1 >= small_frame.rows() {break;}
        for col in 0..small_frame.cols() {
            let pixel0: Vec3b = *small_frame.at_2d(row0, col)?;
            let pixel1: Vec3b = *small_frame.at_2d(row1, col)?;
            let (rf, gf, bf) = (pixel0[2], pixel0[1], pixel0[0]);
            let (rg, gg, bg) = (pixel1[2], pixel1[1], pixel1[0]);
            write!(frame_str, "\x1B[48;2;{};{};{}m\x1B[38;2;{};{};{}m{}", rg, gg, bg, rf, gf, bf, ch).expect("write failed");
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

// ▀ ▄

/// literally only needed because of cheese_grater()
pub fn render(frame: &Mat, term_width: u16, term_height: u16) -> opencv::Result<()> {
    render_frame_hi_res(frame, term_width, term_height, '▀')
}

/// this function is a fucking joke
pub fn cheese_grater(frame: &Mat, term_width: u16, term_height: u16) -> opencv::Result<()> {
    render_frame_hi_res(frame, term_width, term_height, '▄')
}