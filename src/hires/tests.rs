use super::*;
use opencv::core;
use regex::Regex;
fn setup_test_mat(rows: u16, cols: u16) -> Mat {
    let frame: core::MatExpr = core::Mat::zeros(rows as i32, cols as i32, core::CV_8UC3).expect("failed to generate opencv test MatExpr (this is probably a bug in opencv)");
    let frame: Mat = frame.to_mat().expect("can't generate test Mat from test MatExpr (this is probably a bug in opencv)");
    return frame;
}

#[test]
fn test_render_frame() {
    const ROWS: u16 = 3;
    const COLS: u16 = 3;
    const CHAR: char = '▀';
    let frame: Mat = setup_test_mat(ROWS, COLS);

    // our things
    let text: String = render_frame_hi_res(&frame, COLS, ROWS, CHAR).expect("can't unwrap returned string");
    let re = Regex::new(
        r"\x1B\[48;2;(\d+);(\d+);(\d+)m\x1B\[38;2;(\d+);(\d+);(\d+)m(.)")
        .expect("regex fialed to build");

    for caps in re.captures_iter(&text) {
        let bg = (
            caps[1].parse::<u8>().unwrap(),
            caps[2].parse::<u8>().unwrap(),
            caps[3].parse::<u8>().unwrap(),
        );

        let fg = (
            caps[4].parse::<u8>().unwrap(),
            caps[5].parse::<u8>().unwrap(),
            caps[6].parse::<u8>().unwrap(),
        );

        let ch = caps[7].chars().next().unwrap();
        
        assert_eq!(bg, (0, 0 ,0)); // all black images
        assert_eq!(fg, (0, 0 ,0));
        assert_eq!(ch, CHAR);
    }
}
