use std::{
    fmt::Write as FmtWrite, io::{stdout, Write}, time::Duration
};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

pub fn duration_from_fps(fps: f64) -> Duration {
    assert!(fps > 0.0, "Invalid FPS: {}", fps);
    return Duration::from_secs_f64(1.0 / fps);
}

pub fn render_frame_hi_res(frame: &Mat, term_width: u16, term_height: u16) -> opencv::Result<()> {
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
            write!(frame_str, "\x1B[48;2;{};{};{}m\x1B[38;2;{};{};{}m▀", rg, gg, bg, rf, gf, bf).expect("write failed");
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

pub fn render_frame_lo_res(frame: &Mat, term_width: u16, term_height: u16) -> opencv::Result<()> {
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

#[macro_export]
macro_rules! print_status_bar {
    (
        $frame_read_duration:expr,
        $loop_duration:expr,
        $sleep_time:expr,
        $fps:expr,
        $frame_number:expr,
        $audio_enabled:expr
    ) => {
        {
            let debug_read: f64 = $frame_read_duration.as_secs_f64() * 1000.0;
            let debug_loop: f64 = $loop_duration.as_secs_f64() * 1000.0;
            let debug_sleep: f64 = $sleep_time.as_secs_f64() * 1000.0;
            print!(
                "\x1B[0mread: {:.3}ms, draw: {:.3}ms, sleep: {:.3}ms, total: {:.3}ms, targetfps: {:.3}, frame#: {}, audio: {:?}",
                debug_read,
                debug_loop,
                debug_sleep,
                debug_read + debug_loop + debug_sleep,
                $fps,
                $frame_number,
                $audio_enabled
            );
        }
    };
}
