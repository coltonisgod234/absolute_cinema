//! SIXEL rendering core

use std::fmt::Write;
use opencv::core::Vec3b;

/// initializes sixel for black and white drawing:
/// - moves the cursor to the home position
/// - enters sixel mode
/// - defines colour register `1` as 100%, 100%, 100% (pure white)
/// - switches to colour register `1`
pub const BEGIN_SIXEL_BW: &str = "\x1b[H\x1BPq#1;2;100;100;100#1";

/// Enters colour sixel mode.
/// 
/// Defines no colour registers, does not move the cursor,
/// just enters sixel mode
pub const BEGIN_SIXEL_COLOUR: &str = "\x1BP1;0q";

/// exits sixel mode
pub const END_SIXEL: & str= "\x1B\\";

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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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

pub fn determine_start_string_with_offset(colours: &Vec<(u8,u8,u8)>, off: usize) -> String {
    let mut output = String::new();
    for (mut idx, colour) in colours.iter().enumerate() {
        idx += off;
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

/// Calculates the actual difference
#[allow(dead_code)]
pub fn closest_colour_index_rgb_real(target: (u8, u8, u8), colours: &[(u8, u8, u8)]) -> Option<usize> {
    colours.iter().enumerate().min_by_key(|&(_, &(r, g, b))| {
        let dr = r as i32 - target.0 as i32;
        let dg = g as i32 - target.1 as i32;
        let db = b as i32 - target.2 as i32;
        dr * dr + dg * dg + db * db
    }).map(|(i, _)| i)
}

/// Calculates the perceptual difference (using perceptual weights)
pub fn closest_colour_index_rgb(target: (u8, u8, u8), colours: &[(u8, u8, u8)]) -> Option<usize> {
    colours.iter().enumerate().min_by_key(|&(_, &(r, g, b))| {
        let dr = r as i32 - target.0 as i32;
        let dg = g as i32 - target.1 as i32;
        let db = b as i32 - target.2 as i32;
        dr*dr*30 + dg*dg*11 + db*db*59
    }).map(|(i, _)| i)
}

// ////////////////////////////////////////////////////////////////////////////
// HSV BULLSHIT!
// ////////////////////////////////////////////////////////////////////////////

/// Converts RGB color to HSV color space.
///
/// # Arguments
/// * `rgb` - Tuple `(r, g, b)` with each component in `0.0..1.0`.
///
/// # Returns
/// Tuple `(h, s, v)`:
/// - `h`: hue in degrees `[0.0, 360.0)`. Red = 0°, green = 120°, blue = 240°.
/// - `s`: saturation `[0.0, 1.0]`. 0 = grey, 1 = fully saturated.
/// - `v`: value `[0.0, 1.0]`. 0 = black, 1 = brightest.
///
/// # Notes
/// - For greys (`r = g = b`), hue is defined as `0.0`.
#[inline]
#[allow(dead_code)]
pub fn rgb_to_hsv(rgb: (f32, f32, f32)) -> (f32, f32, f32) {
    let (r, g, b) = rgb;

    // Max and min channels
    let cmax = r.max(g).max(b);
    let cmin = r.min(g).min(b);
    let delta = cmax - cmin;

    // Value
    let v = cmax;

    // Saturation
    let s = if cmax == 0.0 { 0.0 } else { delta / cmax };

    // Hue calculation
    let mut h = if delta == 0.0 {
        0.0 // grey
    } else if cmax == r {
        ((g - b) / delta).rem_euclid(6.0)
    } else if cmax == g {
        ((b - r) / delta) + 2.0
    } else {
        ((r - g) / delta) + 4.0
    };

    h *= 60.0; // convert from “sector number” to degrees
    if h < 0.0 { h += 360.0; }

    (h, s, v)
}

/// Finds the index of the palette color closest to `target` in HSV space.
///
/// # Arguments
/// * `target` - HSV color `(h, s, v)` with `h` in degrees `[0,360)`, `s` and `v` in `[0,1]`.
/// * `colours` - Slice of HSV palette colors `(h, s, v)`.
///
/// # Returns
/// Option containing the index of the closest palette color.
///
/// # Notes
/// - Distance metric is a **weighted sum**:
///     - Hue difference wrapped around 360° (0.5×)
///     - Saturation difference (1×)
///     - Value difference (1×)
/// - For greys (`s < 0.05`), hue is ignored to prevent greys from stealing vibrant colors.
/// - Safe against NaNs: `partial_cmp` fallback used.
#[allow(dead_code)]
pub fn closest_colour_index_hsv(target: (f32, f32, f32), colours: &[(f32, f32, f32)]) -> Option<usize> {
    let (h_t, s_t, v_t) = target;

    colours.iter().enumerate()
        .min_by(|&(_, &(h1,s1,v1)), &(_, &(h2,s2,v2))| {
            // Hue distance, wrap around 360
            let dh1 = ((h1 - h_t).abs()).min(360.0 - (h1 - h_t).abs());
            let dh2 = ((h2 - h_t).abs()).min(360.0 - (h2 - h_t).abs());

            // Saturation and value differences
            let ds1 = (s1 - s_t).abs();
            let ds2 = (s2 - s_t).abs();
            let dv1 = (v1 - v_t).abs();
            let dv2 = (v2 - v_t).abs();

            // // Ignore hue if either color is grey
            // let dh1 = if s1 < 0.001 || s_t < 0.001 { 0.0 } else { dh1 };
            // let dh2 = if s2 < 0.001 || s_t < 0.001 { 0.0 } else { dh2 };

            // Weighted sum of differences
            let d1 = dh1 * 2.0 + ds1 * 1.0 + dv1 * 0.5;
            let d2 = dh2 * 2.0 + ds2 * 1.0 + dv2 * 0.5;

            d1.partial_cmp(&d2).unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|(i, _)| i)
}

// ////////////////////////////////////////////////////////////////////////////

/// Writes a u8 to a string without allocating
pub fn write_u8_to_str_noalloc(x: u8, s: &mut String) {
    // recursive base case: single digit form
    if x < 10 {
        let c = (x + b'0') as char;
        s.push(c);
        return;
    }

    else if x < 100 {
        let tens_digit = (x / 10) % 10;
        let ones_digit = x % 10;
        write_u8_to_str_noalloc(tens_digit, s);
        write_u8_to_str_noalloc(ones_digit, s);
        return;
    }

    else {
        // no need to mod 100 here, it's already either 1 or 2
        let hundreds_digit = x / 100;
        let tens_digit = (x / 10) % 10;
        let ones_digit = x % 10;
        write_u8_to_str_noalloc(hundreds_digit, s);
        write_u8_to_str_noalloc(tens_digit, s);
        write_u8_to_str_noalloc(ones_digit, s);
        return;
    }
}

#[allow(dead_code)]
pub fn write_usize_noalloc(mut x: usize, s: &mut String) {
    let mut buf = [0u8; 20]; // enough for 64-bit usize
    let mut i = buf.len();

    if x == 0 {
        s.push('0');
        return;
    }

    while x != 0 {
        i -= 1;
        buf[i] = b'0' + (x % 10) as u8;
        x /= 10;
    }

    unsafe {
        s.push_str(std::str::from_utf8_unchecked(&buf[i..]));
    }
}

#[cfg(test)]
mod tests;
