use opencv::core::Vec3b;

pub fn calc_brightness(pixel: Vec3b) -> u8 {
    return (
        0.299 * pixel[2] as f32
        + 0.587 * pixel[1] as f32
        + 0.114 * pixel[0] as f32) as u8;
}

pub fn find_codepoint(pixels: [bool;8]) -> char {
    let mut bits = 0u8;
    for (idx, on) in pixels.iter().enumerate() {
        if *on {
            bits |= 1 << idx;
        }
    }
    return char::from_u32(0x2800 + bits as u32).expect("expected codepoint")
}

pub fn pixel_on(brightness: u8, threshold: u8) -> bool {
    return brightness > threshold
}
