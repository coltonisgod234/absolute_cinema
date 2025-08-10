use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use clap::Parser;
use crossterm::terminal;
use opencv::{
    prelude::*,
    videoio::{VideoCapture, CAP_FFMPEG, CAP_PROP_FPS, CAP_PROP_POS_FRAMES},
};
mod audio;
mod video;

#[derive(Parser)]
struct Cli {
    path: String,

    #[arg(short='a', long="no-audio", help="disable audio")]
    no_audio: bool,

    #[arg(short='l', long="low-resolution", help="don't use block characters to increase vertical resolution")]
    low_resolution: bool,

    #[arg(short='s', long="no-status-bar", help="don't print the status bar")]
    no_status_bar: bool,

    #[arg(short='c', long="clear-screen", help="clear the screen before drawing anything")]
    clear_screen: bool
}

fn main() -> opencv::Result<()> {
    let args: Cli = Cli::parse();
    // Debug: show OpenCV build info
    let video_path: &str = args.path.as_str();
    println!("{}", opencv::core::get_build_information()?);

    // Open the video file
    let mut cap: VideoCapture = VideoCapture::from_file(
        video_path,
        CAP_FFMPEG,
    )?;
    //let mut cap: VideoCapture = VideoCapture::from_file("https://www.youtube.com/watch?v=SXySxLgCV-8", CAP_ANY)?;
    if !cap.is_opened()? {
        panic!("Failed to open video file");
    }

    // Get playback parameters
    let fps: f64 = cap.get(CAP_PROP_FPS)?;
    let frame_delay: Duration = video::duration_from_fps(fps);
    let (term_width, term_height) = terminal::size().unwrap_or((80, 25));
    let actual_pixels_height: u16 = match args.low_resolution {
        false => (term_height * 2) - 1,
        true => term_height - 1
    };

    let mut frame: Mat = Mat::default();

    // start playing audio (if enabled)
    let _stream: Option<rodio::OutputStream> = if !args.no_audio {
        Some(audio::start_audio(video_path).expect("audio failed to start"))
    } else { None };

    let render_function: fn(&Mat, u16, u16) -> Result<(), opencv::Error> = match args.low_resolution {
        false => video::render_frame_hi_res,
        //false => video::render_frame_for_decs_fucking_retarded_ass_protocol,
        true => video::render_frame_lo_res
    };
    loop {
        let read_start = Instant::now();
        if !cap.read(&mut frame)? || frame.empty() {
            break;
        }
        let read_end = Instant::now();
        let frame_read_duration = read_end - read_start;

        let loop_start = Instant::now();

        if args.clear_screen {
            print!("\x1B[2J");
        }
        render_function(&mut frame, term_width, actual_pixels_height)?;

        let loop_end = Instant::now();
        let loop_duration = loop_end - loop_start;
        let total_loop_duration = frame_read_duration + loop_duration;
        let sleep_time = frame_delay.checked_sub(total_loop_duration).unwrap_or(Duration::ZERO);
        sleep(sleep_time);

        if !args.no_status_bar {
            print_status_bar!(
                frame_read_duration,
                loop_duration,
                sleep_time,
                fps,
                cap.get(CAP_PROP_POS_FRAMES).unwrap_or(-1.0),
                !args.no_audio
            );
        }
    }

    println!("video done, terminating");
    Ok(())
}
