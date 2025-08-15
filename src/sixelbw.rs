use std::io::{stdout, Write};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

pub fn rgb_to_sixel_percent(c: u8) -> u8 {
    ((c as f32 / 255.0) * 100.0).round() as u8
}

pub fn draw_sixel_col(pixels: [bool;6], colour: Vec3b, register: u8) -> opencv::Result<()> {
    let r: u8 = rgb_to_sixel_percent(colour[2]); // OpenCV stores as BGR, so reverse
    let g: u8 = rgb_to_sixel_percent(colour[1]);
    let b: u8 = rgb_to_sixel_percent(colour[0]);
    let color_index = register;  // always 0 right now

    print!("#{};2;{};{};{}", color_index, r, g, b);  // define colour
    print!("#{}", color_index);  // switch to that colour

    // do some "who's that pokemon" ahh bullshit to find the fucking char to print
    let mut bits: u8 = 0u8;
    for (idx, on) in pixels.iter().enumerate() {
        if *on {
            bits |= 1 << idx;
        }
    }

    // print the fucking character
    print!("{}", (bits + 63) as char);
    Ok(())
}

pub fn sixel_is_on_bw(pixel: Vec3b, threshold: u8) -> bool {
    // BGR to brightness
    let brightness: u8 = (0.299 * pixel[2] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[0] as f32) as u8;
    brightness > threshold
}

pub fn make_sixel_render_bw(threshold: u8) -> impl Fn(&Mat, u16, u16) -> opencv::Result<()> {
    move |frame: &Mat, w: u16, h: u16| {
        render(frame, w, h, threshold)
    }
}

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
    print!("\x1b[H"); // move cursor home
    print!("\x1BPq");  // enter sixel
    let height: i32 = small_frame.rows();
    let width: i32 = small_frame.cols();
    let mut stdout = stdout();

    // what the fuck is happening anymore
    // for each vertical band of 6 pixels
    for y_start in (0..height).step_by(6) {
        let colour_reg = 1;  // FIXED for this band
        for x in 0..width {
            let mut pixels: [bool; 6] = [false; 6];
            let final_colour = Vec3b::from([255, 255, 255]); 

            // collect pixels for this vertical band column
            for i in 0..6 {
                let y = y_start + i;
                if y < height {
                    let pixel_val: Vec3b = *small_frame.at_2d::<Vec3b>(y, x)?;
                    pixels[i as usize] = sixel_is_on_bw(pixel_val, threshold);
                    //final_colour = Vec3b::from([255, 255, 255]); 
                }
            }
            // Use the same colour register for all columns in this band
            // Use some fixed colour for now, or pick from your palette
            draw_sixel_col(pixels, final_colour, colour_reg)?;
            //stdout.flush().unwrap();
        }
        print!("-");
    }

    print!("\x1b\\");  // exit sixel
    stdout.flush().expect("stdout flush failed");
    Ok(())
}
