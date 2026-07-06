use opencv::{
    prelude::*,
};
use rayon::iter::IndexedParallelIterator;
use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use crate::video::*;
use crate::sixel::*;

pub struct SixelMultipassHQ {
    pub colours: Vec<(u8,u8,u8)>,

    palette_lut: RgbPaletteLUT,
    start_string: String
}

impl SixelMultipassHQ {
    pub fn new(colours: Vec<(u8,u8,u8)>) -> Self {
        let renderer = Self {
            start_string: determine_start_string_with_offset(&colours, 0),
            palette_lut: RgbPaletteLUT::ciede2000_labgen(&colours),
            colours: colours
        };

        return renderer
    }

    fn start_sixel(&self, output: &mut String) {
        output.push_str("\x1B[H");      // move cursor to top-left
        output.push_str(BEGIN_SIXEL_COLOUR);
        output.push_str(&self.start_string);
    }

    fn stop_sixel(&self, output: &mut String) {
        output.push_str(END_SIXEL);
    }

    fn do_sixel_pass(&self, wid: i32, hei: i32, colour: PaletteIndex, v: &Vec<Vec<PaletteIndex>>) -> String {
        let mut buff = String::new();
        self.start_sixel(&mut buff);
        
        buff.push('#');
        write_u8_to_str_noalloc(colour, &mut buff);

        for y_start in (0..hei).step_by(6) {
            for x in 0..wid {
                let mut sixels: [bool; 6] = [false; 6];

                // basically just check if the pallete index at this position is the one we care about
                // and if it is, then set it.
                for dy in 0..6 {
                    let y = y_start + dy;
                    if let Some(inner_v) = v.get(x as usize) {
                        if let Some(pidx) = inner_v.get(y as usize) {
                            if *pidx == colour {
                                sixels[dy as usize] = true;
                            }
                        }
                    }
                }

                buff.push(calculate_sixel_cols(sixels))
            }

            buff.push('-');
        }

        self.stop_sixel(&mut buff);

        return buff
    }
}

impl Renderer for SixelMultipassHQ {
    fn draw(&mut self, frame: &Mat) -> opencv::Result<Box<dyn Renderable>> {
        let hei = frame.rows();
        let wid = frame.cols();

        // Assuming wid and hei are usize
        let v: Vec<Vec<PaletteIndex>> = (0..wid)
            .into_par_iter() // parallelize the outer loop
            .map(|x| {
                let mut inner_v: Vec<PaletteIndex> = Vec::with_capacity(hei as usize);
                for y in 0..hei {
                    let px = get_px(&frame, x, y as i32);
                    inner_v.push(
                        self.palette_lut.get_bgr(px.as_slice())
                    );
                }
                inner_v
            })
            .collect(); // collects into Vec<Vec<PaletteIndex>>

        let outs: Vec<String> = self.colours
            .par_iter()
            .enumerate()
            .map(|(i, _)| self.do_sixel_pass(wid, hei, i as u8, &v))
            .collect();

        return Ok(Box::new(outs.concat()))
    }
}
