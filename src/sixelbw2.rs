use std::io::{stdout, Write};
use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};
use crate::sixelbw::{
    sixel_is_on_bw,
    print_pixels,
    begin_sixel_bw,
    end_sixel_bw
};

/// see `sixelbw::make_sixel_render_bw`
pub fn make_sixel_render_bw2(adjust: i8) -> impl Fn(&Mat, u16, u16) -> opencv::Result<()> {
    move |frame: &Mat, w: u16, h: u16| {
        render(frame, w, h, adjust)
    }
}

/// sixelbw2 produces better results than the sixelbw algorithm at the cost
/// of speed and oftentimes resolution.
/// 
/// calculates the average brightness of the frame, adjusts it by
/// `adjust`, then compares against that value to determine if the sixel is on
/// or off.
/// 
/// black-and-white only
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
    begin_sixel_bw();

    let height: i32 = small_frame.rows();
    let width: i32 = small_frame.cols();
    let total_px: i32 = width * height;

    let mut stdout = stdout();

    // calculate the average brightness
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

    // for each vertical band of 6 pixels
    for y_start in (0..height).step_by(6) {
        for x in 0..width {
            let mut pixels: [bool; 6] = [false; 6];

            // collect pixels for this vertical band column
            for i in 0..6 {
                let y = y_start + i;
                if y < height {
                    let pixel_val: Vec3b = *small_frame.at_2d::<Vec3b>(y, x)?;
                    pixels[i as usize] = sixel_is_on_bw(pixel_val, avg_brightness_acc as u8);
                }
            }
            print_pixels(pixels)?;
        }
        print!("-");  // go down 6 pixels
    }

    end_sixel_bw();
    stdout.flush().expect("stdout flush failed");
    Ok(())
}
