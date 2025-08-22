//! SIXEL rendering code

use std::fmt::Write;
use opencv::core::Vec3b;

/// initializes sixel for black and white drawing:
/// - moves the cursor to the home position
/// - enters sixel mode
/// - defines colour register `1` as 100%, 100%, 100% (pure white)
/// - switches to colour register `1`
pub const BEGIN_SIXEL_BW: &str = "\x1b[H\x1BPq#1;2;100;100;100#1";

/// exits sixel mode
pub const END_SIXEL_BW: & str= "\x1B\\";

/// currently dead code!
/// 
/// converts a value from 0-255 to DEC's stupid percentage system (0%-100%)
pub fn rgb_to_sixel_percent(c: u8) -> u8 {
    ((c as f32 / 255.0) * 100.0).round() as u8
}

/// currenly dead code!
/// 
/// changes the colour in a register and draws to the screen
/// this code is very costly and isn't recomended.
pub fn draw_sixel_colour(colour: Vec3b, register: u8) -> String {
    let r: u8 = rgb_to_sixel_percent(colour[2]); // OpenCV stores as BGR, so reverse
    let g: u8 = rgb_to_sixel_percent(colour[1]);
    let b: u8 = rgb_to_sixel_percent(colour[0]);
    let color_index = register;
    let mut output = String::new();

    // define colour
    write!(output, "#{};2;{};{};{}", color_index, r, g, b)
        .expect("can't write colour reg setup cmd to str");

    // switch to that colour
    write!(output, "#{}", color_index)
        .expect("can't write colour cahnge cmd to str");

    return output
}

/// currently dead code!
/// 
/// COSTLY AS FUCK
/// drwas 
pub fn draw_sixel_col(pixels: [bool;6], colour: Vec3b, register: u8) -> () {
    print!("{}{}", draw_sixel_colour(colour, register), calculate_sixel_cols(pixels));
    return ()
}

/// calculates what character to display from a 6-bit array
pub fn calculate_sixel_cols(pixels: [bool;6]) -> char {
    let mut bits: u8 = 0u8;
    for (idx, on) in pixels.iter().enumerate() {
        if *on {
            bits |= 1 << idx;
        }
    }
    return (bits + 63) as char;
}

/// take an array of 6 pixels and draw them to the screen
/// as sixels to the screen
/// 
/// doesn't update any colour regs
pub fn print_pixels(pixels: [bool;6]) -> opencv::Result<()> {
    print!("{}", calculate_sixel_cols(pixels));
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

pub fn determine_start_string(colours: &Vec<(u8,u8,u8)>) -> String {
    let mut output = String::new();
    for (idx, colour) in colours.iter().enumerate() {
        let string: &str = &format!("#{};2;{};{};{}",
            idx,
            rgb_to_sixel_percent(colour.2),  // R
            rgb_to_sixel_percent(colour.1),  // G
            rgb_to_sixel_percent(colour.0)   // B
        );
        output.push_str(string);
    }
    return output
}

pub fn closest_colour_index(target: (u8, u8, u8), colours: &[(u8, u8, u8)]) -> Option<usize> {
    colours.iter().enumerate().min_by_key(|&(_, &(r, g, b))| {
        let dr = r as i32 - target.0 as i32;
        let dg = g as i32 - target.1 as i32;
        let db = b as i32 - target.2 as i32;
        dr * dr + dg * dg + db * db
    }).map(|(i, _)| i)
}

#[cfg(test)]
mod tests;
