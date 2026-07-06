use std::io::{Stdout, Write};

use opencv::{core::{Mat, MatTraitConst, Vec3b}};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefMutIterator, ParallelIterator};

use crate::sixel::{closest_colour_index_rgb};

pub trait Renderable {
    fn render(&self, stdout: &mut Stdout);
}

pub trait Renderer {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>>;
}

// implement these traits
impl Renderable for String {
    fn render(&self, stdout: &mut Stdout) {
        stdout.write_all(self.as_bytes()).unwrap();
        stdout.flush().unwrap();
    }
}

pub fn get_px(frame: &Mat, x: i32, y: i32) -> Vec3b {
    let data = frame.data(); // u8 slice
    let offset: i32 = (y * frame.cols() + x) * 3;

    unsafe {
        let r = *data.add((offset + 2) as usize);
        let g = *data.add((offset + 1) as usize);
        let b = *data.add(offset as usize);
        Vec3b::from([r, g, b])
    }
}

#[allow(dead_code)]
pub fn normalize_rgb_to_f32s(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
    return (
        (rgb.0 / 255) as f32,
        (rgb.1 / 255) as f32,
        (rgb.2 / 255) as f32,
    )
}

pub type PaletteIndex = u8;

pub const LUT_RGB_SIZE: usize = 256 * 256 * 256;

pub struct RgbPaletteLUT {
    lut: Box<[u8; LUT_RGB_SIZE]>,
}

impl RgbPaletteLUT {
    #[allow(dead_code)]
    pub fn generate(palette: &Vec<(u8, u8, u8)>) -> Self {
        let mut my = Self {
            lut: Box::new([0; LUT_RGB_SIZE])
        };

        my.lut.par_iter_mut().enumerate().for_each(|(i, out)| {
            let r = ((i >> 16) & 0xFF) as u8;
            let g = ((i >> 8) & 0xFF) as u8;
            let b = (i & 0xFF) as u8;

            *out = closest_colour_index_rgb((r, g, b), &palette)
                .expect("failed to generate LUT")
                as u8;
        });

        return my
    }

    /// Generates the LUT while converting to HSV and back for each colour,
    /// which yields a VERY ACCURATE (or rather, visually pleasing) LUT, but
    /// takes slightly longer to generate.
    #[allow(unused)]
    pub fn generate_via_hsv(rgb_palette: &Vec<(u8, u8, u8)>) -> Self {
        use crate::sixel::{rgb_to_hsv, closest_colour_index_hsv};

        let mut my = Self {
            lut: Box::new([0; LUT_RGB_SIZE])
        };

        // Precompute palette in float HSV
        let palette_hsv_f: Vec<(f32,f32,f32)> = rgb_palette.iter()
            .map(|&(r,g,b)| (normalize_rgb_to_f32s((r,g,b))))
            .collect();

        my.lut.par_iter_mut().enumerate().for_each(|(i, out)| {
            let r = ((i >> 16) & 0xFF) as u8;
            let g = ((i >> 8) & 0xFF) as u8;
            let b = (i & 0xFF) as u8;

            let normal_colours = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

            let hsv = rgb_to_hsv(normal_colours);

            // Use float HSV for matching
            *out = closest_colour_index_hsv(hsv, &palette_hsv_f)
                .expect("failed to generate LUT") as u8;
        });


        return my
    }

    pub fn ciede2000_labgen(palette: &Vec<(u8, u8, u8)>) -> Self {
        let mut the = Self {
            lut: Box::new([0; LUT_RGB_SIZE])
        };

        let palette_lab: Vec<[f32;3]> = palette.iter()
            .map(|&(r,g,b)| rgb_to_lab(r,g,b))
            .collect();

        the.lut.par_iter_mut().enumerate().for_each(|(i, out)| {
            let r = ((i >> 16) & 0xFF) as u8;
            let g = ((i >> 8) & 0xFF) as u8;
            let b = (i & 0xFF) as u8;

            let lab = rgb_to_lab(r, g, b);

            // find closest palette index
            let mut best_idx = 0;
            let mut best_dist = f32::MAX;

            for (idx, p_lab) in palette_lab.iter().enumerate() {
                // let dl = lab[0] - p_lab[0];
                // let da = lab[1] - p_lab[1];
                // let db = lab[2] - p_lab[2];
                // let weight = 1.5;
                // let dist = dl.powi(2) + (da*weight).powi(2) + (db*weight).powi(2);

                // more better??
                let l = lab[0] + 5.0;         // bump lightness by +5
                let a = lab[1] * 1.1;         // increase a-channel (green–red) 10%
                let b = lab[2] * 1.1;         // increase b-channel (blue–yellow) 10%
                let dist = delta_e_ciede2000([l, a, b], *p_lab);

                if dist < best_dist {
                    best_dist = dist;
                    best_idx = idx;
                }
            }

            *out = best_idx as u8;
        });

        return the;
    }

    pub fn get_bgr(&self, colour: &[u8]) -> PaletteIndex {
        let idx =
            ((colour[2] as usize) << 16) |
            ((colour[1] as usize) << 8)  |
            (colour[0] as usize);

        return self.lut[idx]
    }
}

pub fn delta_e_ciede2000(lab1: [f32;3], lab2: [f32;3]) -> f32 {
    let (l1, a1, b1) = (lab1[0], lab1[1], lab1[2]);
    let (l2, a2, b2) = (lab2[0], lab2[1], lab2[2]);

    // 1. Compute C* and h*
    let c1 = (a1*a1 + b1*b1).sqrt();
    let c2 = (a2*a2 + b2*b2).sqrt();
    let c_avg = (c1 + c2) / 2.0;

    // 2. Adjust a' using G
    let g = 0.5 * (1.0 - (c_avg.powi(7) / (c_avg.powi(7) + 25f32.powi(7))).sqrt());
    let a1_prime = a1 * (1.0 + g);
    let a2_prime = a2 * (1.0 + g);

    let c1_prime = (a1_prime*a1_prime + b1*b1).sqrt();
    let c2_prime = (a2_prime*a2_prime + b2*b2).sqrt();

    let h1_prime = b1.atan2(a1_prime).to_degrees().rem_euclid(360.0);
    let h2_prime = b2.atan2(a2_prime).to_degrees().rem_euclid(360.0);

    // 3. ΔL', ΔC', ΔH'
    let delta_l_prime = l2 - l1;
    let delta_c_prime = c2_prime - c1_prime;

    let mut delta_h_prime = h2_prime - h1_prime;
    if delta_h_prime.abs() > 180.0 {
        delta_h_prime -= 360.0 * delta_h_prime.signum();
    }
    let delta_h_prime = 2.0 * (c1_prime * c2_prime).sqrt() * (delta_h_prime.to_radians()/2.0).sin();

    // 4. Compute weighting functions
    let l_avg = (l1 + l2) / 2.0;
    let c_avg_prime = (c1_prime + c2_prime) / 2.0;

    let h_avg_prime = if (h1_prime - h2_prime).abs() > 180.0 {
        (h1_prime + h2_prime + 360.0)/2.0
    } else {
        (h1_prime + h2_prime)/2.0
    };

    let t = 1.0
        - 0.17 * ((h_avg_prime - 30.0).to_radians().cos())
        + 0.24 * ((2.0 * h_avg_prime).to_radians().cos())
        + 0.32 * ((3.0 * h_avg_prime + 6.0).to_radians().cos())
        - 0.20 * ((4.0 * h_avg_prime - 63.0).to_radians().cos());

    let delta_theta = 30.0 * (-((h_avg_prime - 275.0)/25.0).powi(2)).exp();
    let r_c = 2.0 * (c_avg_prime.powi(7) / (c_avg_prime.powi(7) + 25f32.powi(7))).sqrt();
    let s_l = 1.0 + ((0.015 * (l_avg - 50.0).powi(2)) / ((20.0 + (l_avg - 50.0).powi(2)).sqrt()));
    let s_c = 1.0 + 0.045 * c_avg_prime;
    let s_h = 1.0 + 0.015 * c_avg_prime * t;
    let r_t = -r_c * (2.0 * delta_theta.to_radians()).sin();

    // 5. Base ΔE00
    let mut delta_e = ((delta_l_prime / s_l).powi(2)
                 + (delta_c_prime / s_c).powi(2)
                 + (delta_h_prime / s_h).powi(2)
                 + r_t * (delta_c_prime / s_c) * (delta_h_prime / s_h)).sqrt();

    // 6. Boost chroma slightly for green-ish colors to prevent dulling
    // You can tweak factor (e.g., 0.1–0.2) as needed
    let chroma_boost_factor = 0.15;
    if a2 >= -50.0 && a2 <= 50.0 && b2 >= 50.0 { // roughly green region in Lab
        delta_e *= 1.0 - chroma_boost_factor;
    }

    delta_e
}

// simple handrolled RGB -> LAB
fn rgb_to_lab(r: u8, g: u8, b: u8) -> [f32;3] {
    // normalize 0..1
    let r = r as f32 / 255.0;
    let g = g as f32 / 255.0;
    let b = b as f32 / 255.0;

    // linearize
    let r = if r > 0.04045 { ((r+0.055)/1.055).powf(2.4) } else { r / 12.92 };
    let g = if g > 0.04045 { ((g+0.055)/1.055).powf(2.4) } else { g / 12.92 };
    let b = if b > 0.04045 { ((b+0.055)/1.055).powf(2.4) } else { b / 12.92 };

    // RGB -> XYZ
    let x = r*0.4124564 + g*0.3575761 + b*0.1804375;
    let y = r*0.2126729 + g*0.7151522 + b*0.0721750;
    let z = r*0.0193339 + g*0.1191920 + b*0.9503041;

    // normalize by D65 white
    let x = x / 0.95047;
    let y = y / 1.0;
    let z = z / 1.08883;

    // f(t)
    let fx = if x > 0.008856 { x.powf(1.0/3.0) } else { 7.787*x + 16.0/116.0 };
    let fy = if y > 0.008856 { y.powf(1.0/3.0) } else { 7.787*y + 16.0/116.0 };
    let fz = if z > 0.008856 { z.powf(1.0/3.0) } else { 7.787*z + 16.0/116.0 };

    let l = 116.0*fy - 16.0;
    let a = 500.0*(fx - fy);
    let b = 200.0*(fy - fz);

    [l,a,b]
}