use std::io::{stdout, Write};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};
use crate::sixelbw::{
    sixel_is_on_bw,
    draw_sixel_col,
};

pub fn make_sixel_render_bw2(adjust: i8) -> impl Fn(&Mat, u16, u16) -> opencv::Result<()> {
    move |frame: &Mat, w: u16, h: u16| {
        render(frame, w, h, adjust)
    }
}

pub fn render(frame: &Mat, term_width: u16, term_height: u16, adjust: i8) -> opencv::Result<()> {
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
    let total_px: i32 = width * height;

    let mut stdout = stdout();

    let mut avg_brightness_acc: i32 = 0;
    for x in 0..width {
        for y in 0..height {
            let pixel: Vec3b = *small_frame.at_2d::<Vec3b>(y, x)?;  // BGR
            let brightness: u8 = (0.299 * pixel[2] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[0] as f32) as u8;
            avg_brightness_acc += brightness as i32;
        }
    }
    avg_brightness_acc /= total_px as i32;
    avg_brightness_acc += adjust as i32;

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
                    pixels[i as usize] = sixel_is_on_bw(pixel_val, avg_brightness_acc as u8);
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
