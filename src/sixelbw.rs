use std::io::{stdout, Write};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

/// initializes sixel for black and white drawing:
/// - moves the cursor to the home position
/// - enters sixel mode
/// - defines colour register `1` as 100%, 100%, 100% (pure white)
/// - switches to colour register `1`
pub fn begin_sixel_bw() {
    print!("\x1b[H"); // move cursor home
    print!("\x1BPq");  // enter sixel
    print!("#1;2;100;100;100");  // use white as paint
    print!("#1");  // switch to white
}

/// exits sixel mode
pub fn end_sixel_bw() {
    print!("\x1B\\");
}

/// currently dead code!
/// 
/// converts a value from 0-255 to DEC's stupid percentage system (0%-100%)
pub fn rgb_to_sixel_percent(c: u8) -> u8 {
    ((c as f32 / 255.0) * 100.0).round() as u8
}

/// currently dead code!
/// 
/// changes the colour in a register and draws to the screen
/// this code is very costly and isn't recomended.
pub fn draw_sixel_col(pixels: [bool;6], colour: Vec3b, register: u8) -> opencv::Result<()> {
    let r: u8 = rgb_to_sixel_percent(colour[2]); // OpenCV stores as BGR, so reverse
    let g: u8 = rgb_to_sixel_percent(colour[1]);
    let b: u8 = rgb_to_sixel_percent(colour[0]);
    let color_index = register;  // always 0 right now

    print!("#{};2;{};{};{}", color_index, r, g, b);  // define colour
    print!("#{}", color_index);  // switch to that colour

    return print_pixels(pixels)
}

/// take an array of 6 pixels and preform the math needed to draw them
/// as sixels to the screen
/// 
/// doesn't update any colour regs
pub fn print_pixels(pixels: [bool;6]) -> opencv::Result<()> {
    let mut bits: u8 = 0u8;
    for (idx, on) in pixels.iter().enumerate() {
        if *on {
            bits |= 1 << idx;
        }
    }
    
    print!("{}", (bits + 63) as char);
    Ok(())
}

/// function to check if a sixel is on.
/// specific to this module's render()
pub fn sixel_is_on_bw(pixel: Vec3b, threshold: u8) -> bool {
    // BGR to brightness
    let brightness: u8 = (
        0.299 * pixel[2] as f32
        + 0.587 * pixel[1] as f32
        + 0.114 * pixel[0] as f32) as u8;

    brightness > threshold
}

/// creates a closure that calls this module's render() with correct params
pub fn make_sixel_render_bw(threshold: u8) -> impl Fn(&Mat, u16, u16) -> opencv::Result<()> {
    move |frame: &Mat, w: u16, h: u16| {
        render(frame, w, h, threshold)
    }
}

/// the star of the show
/// 
/// outputs stuff as sixel!
/// black-and-white only
pub fn render(frame: &Mat, term_width: u16, term_height: u16, threshold: u8) -> opencv::Result<()> {
    let mut small_frame = Mat::default();
    resize(
        frame,
        &mut small_frame,
        opencv::core::Size { width: term_width as i32, height: term_height as i32 },
        0.0,
        0.0,
        INTER_LINEAR,
    )?;
    begin_sixel_bw();

    let height: i32 = small_frame.rows();
    let width: i32 = small_frame.cols();
    let mut stdout = stdout();

    // what the fuck is happening anymore
    // for each vertical band of 6 pixels
    for y_start in (0..height).step_by(6) {
        for x in 0..width {
            let mut pixels: [bool; 6] = [false; 6];

            // collect pixels for this vertical band column
            for i in 0..6 {
                let y = y_start + i;
                if y < height {
                    let pixel_val: Vec3b = *small_frame.at_2d::<Vec3b>(y, x)?;
                    pixels[i as usize] = sixel_is_on_bw(pixel_val, threshold);
                }
            }
            // Use the same colour register for all columns in this band
            // Use some fixed colour for now, or pick from your palette
            //draw_sixel_col(pixels, final_colour, colour_reg)?;
            print_pixels(pixels)?;
            //stdout.flush().unwrap();
        }
        print!("-");
    }

    end_sixel_bw();
    stdout.flush().expect("stdout flush failed");
    Ok(())
}
