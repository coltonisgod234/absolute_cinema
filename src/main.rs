use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use clap::Parser;
use crossterm::terminal;
use opencv::{
    imgproc::{resize, INTER_NEAREST},
    prelude::*,
    videoio::{VideoCapture, CAP_FFMPEG, CAP_PROP_FPS, CAP_PROP_POS_FRAMES}
};

use crate::video::{Renderable, Renderer};
mod audio;

// graphics modes
mod hires;
mod lowres;

// SIXEL rendering
mod sixel;
mod sixelbw;
mod sixelbw2;
mod sixelrgb;

// braille rendering
mod braille;
mod braillebw;
mod braillergb;

mod statusbar;
mod video;

#[derive(Parser)]
struct Cli {
    path: String,

    #[arg(short='a', long="no-audio", help="disable audio")]
    no_audio: bool,

    #[arg(short='g', long="graphics", help="graphics mode", default_value="high")]
    graphics: String,

    #[arg(short='W', long="width", help="resize the image to this width")]
    width: Option<u16>,

    #[arg(short='H', long="height", help="resize the image to this height")]
    height: Option<u16>,

    #[arg(short='s', long="no-status-bar", help="don't print the status bar")]
    no_status_bar: bool,

    // graphics options
    #[arg(long="high-char", help="picks the char for high graphics mode", default_value_t='▀')]
    high_graphics_character: char,

    #[arg(long="low-char", help="picks the char for low graphics mode", default_value_t=' ')]
    low_graphics_character: char,

    #[arg(long="threshold", help="only valid with sixelbw graphics mode", default_value_t=127)]
    threshold: u8,

    #[arg(long="adjust", help="only valid with sixelbw2 graphics mode", default_value_t=0)]
    adjust: i8,

    #[arg(long="adjust-alpha", help="only valid with sixelbw2 graphics mode", default_value_t=0.5)]
    alpha: f32
}

/// determines the `Duration` to wait from a target `fps`
fn duration_from_fps(fps: f64) -> Duration {
    assert!(fps > 0.0, "Invalid FPS: {}", fps);
    return Duration::from_secs_f64(1.0 / fps);
}

/// determines an appropriate image size based on the command line args
fn autodetect_term_size(args: &Cli) -> (u16, u16) {
    let (mut term_width, mut term_height) = terminal::size()
        .unwrap_or((80, 25));

    term_height = match args.graphics.as_ref() {
        "high" | "cheesegrater" => (term_height * 2) - 1,
        "braillebw" | "braillergb" => (term_height * 2) - 1,
        "sixelbw" | "sixelbw2" => (term_height * 6) - 1,
        _ => term_height
    };

    term_width = match args.graphics.as_ref() {
        "braillebw" | "braillergb" => term_width * 4,
        _ => term_width
    };

    if !args.no_status_bar { // if using status bar decrease by 1 to make room
        term_height -= 1;
    }
    return (term_width, term_height);
}

fn main() -> opencv::Result<()> {
    let args: Cli = Cli::parse();
    let video_path: &str = args.path.as_str();

    // Open the video file
    let mut cap: VideoCapture = VideoCapture::from_file(
        video_path,
        CAP_FFMPEG,
    )?;
    if !cap.is_opened()? {
        panic!("Failed to open video file");
    }

    // Get playback parameters
    let fps: f64 = cap.get(CAP_PROP_FPS)?;
    let frame_delay: Duration = duration_from_fps(fps);

    // set width/height for frames
    let term_width: u16 = args.width.unwrap_or_else(|| autodetect_term_size(&args).0);
    let term_height: u16 = args.height.unwrap_or_else(|| autodetect_term_size(&args).1);

    // pick a render function to use
    let mut renderer: Box<dyn Renderer> = match args.graphics.as_str() {
        "braillebw" => Box::new(braillebw::Braillebw {
            threshold: args.threshold
        }),
        "braillergb" => Box::new(braillergb::BrailleRGB {
            threshold: args.threshold
        }),
        "high" => Box::new(hires::HighRes {
            ch: args.high_graphics_character
        }),
        "low" => Box::new(lowres::LowRes {
            ch: args.low_graphics_character
        }),
        "sixelbw" => Box::new(sixelbw::SixelMono {
            threshold: args.threshold
        }),
        "sixelbw2" => Box::new(sixelbw2::SixelMono2::new(
            args.adjust,
            args.alpha
        )),
        "sixelrgb" => Box::new(sixelrgb::SixelGreyscale::new(
            vec![
                (000,000,000),  // black

                // some colours
                (127,000,000),  // dark red
                (255,000,000),  // bright red
                (000,127,000),  // dark green
                (000,255,000),  // bright green
                (000,000,127),  // dark blue
                (000,000,255),  // bright blue

                // colour combinations
                (127,000,127),  // purple I think??
                (255,000,255),  // PURPLE????

                (127,127,000),  // orange I think??
                (255,255,000),  // ORANGE????

                (000,127,127),  // some weird "aqua"
                (000,255,255),  // ?????????????????????????

                (255,255,255),  // white
            ],
            args.adjust,
            args.alpha
        )),
        "cheesegrater" => Box::new(hires::HighRes {
            ch: '▄'
        }),
        _ => panic!("unrecognized renderer")
    };

    // create a new frame
    let mut orig_frame: Mat = Mat::default();
    let mut frame: Mat = Mat::default();  // resized frame

    // start playing audio (if enabled)
    let _stream: Option<rodio::OutputStream> = if !args.no_audio {
        Some(audio::start_audio(video_path).expect("audio failed to start"))
    } else { None };

    // start drawing shit
    loop {
        // read the frame
        let read_start = Instant::now();
        if !cap.read(&mut orig_frame)? || orig_frame.empty() {
            eprintln!("failed to read (is the video done?)");
            break;
        }
        let read_end = Instant::now();
        let frame_read_duration = read_end - read_start;

        // the loop actually starts here
        let loop_start = Instant::now();

        // resize it to the user's wishes
        resize(
            &orig_frame,
            &mut frame,
            opencv::core::Size { width: term_width as i32, height: term_height as i32 },
            0.0,
            0.0,
            INTER_NEAREST,
        )?;

        // draw the frame
        let output: Box<dyn Renderable> = renderer.draw(&frame)?;
        output.render();

        let loop_end = Instant::now();
        let loop_duration = loop_end - loop_start;
        let total_loop_duration = frame_read_duration + loop_duration;
        let sleep_time = frame_delay.checked_sub(total_loop_duration).unwrap_or(Duration::ZERO);
        
        // wait
        sleep(sleep_time);

        // draw the status bar
        if !args.no_status_bar {
            statusbar::draw_status_bar(
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

// unit testing
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_duration_from_fps() {
        let test_60: Duration = duration_from_fps(60.0);
        assert_eq!(test_60.as_millis(), 16);

        let test_30: Duration = duration_from_fps(30.0);
        assert_eq!(test_30.as_millis(), 33);
    }
}