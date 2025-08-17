use opencv::{
    core::Vec3b,
    imgproc::{resize, INTER_LINEAR},
    prelude::*,
};

fn calc_brightness(pixel: Vec3b) -> u8 {
    return (
        0.299 * pixel[2] as f32
        + 0.587 * pixel[1] as f32
        + 0.114 * pixel[0] as f32) as u8;
}

fn find_codepoint(pixels: [bool;8]) -> char {
    let mut bits = 0u8;
    for (idx, on) in pixels.iter().enumerate() {
        if *on {
            bits |= 1 << idx;
        }
    }
    return char::from_u32(0x2800 + bits as u32).expect("expected codepoint")
}

fn pixel_on(brightness: u8, threshold: u8) -> bool {
    return brightness > threshold
}

pub fn make_render(frame: &Mat, width: u16, height: u16) -> opencv::Result<()> {
    print!("\x1b[H");
    print!("{}", render(frame, width, height).unwrap());
    Ok(())
}

pub fn render(frame: &Mat, width: u16, height: u16) -> opencv::Result<String> {
    let mut small_frame = Mat::default();
    resize(
        frame,
        &mut small_frame,
        opencv::core::Size { width: width as i32, height: height as i32 },
        0.0,
        0.0,
        INTER_LINEAR,
    )?;
    let black: Vec3b = Vec3b::from([0,0,0]);
    let rows: i32 = small_frame.rows();
    let cols: i32 = small_frame.cols();
    let mut output = String::new();
    for row_start in (0..rows).step_by(2) {
        for col_start in (0..cols).step_by(4) {
            let mut pixels: [bool; 8] = [false;8];
            for dx in 0..4 {
                for dy in 0..2 {
                    let pixel_y: i32 = row_start + dy;
                    let pixel_x: i32 = col_start + dx;
                    let pixel: Vec3b = if pixel_y < rows && pixel_x < cols {
                        *small_frame.at_2d::<Vec3b>(pixel_y, pixel_x)?
                    } else { black };
                    let brightness: u8 = calc_brightness(pixel);
                    pixels[dy as usize * 2 + dx as usize] = pixel_on(brightness, 127);
                }
            }
            let ch: char = find_codepoint(pixels);
            output.push(ch);
        }
        output.push('\n');
    }
    Ok(output)
}
