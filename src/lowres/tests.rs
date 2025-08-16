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
    let frame: Mat = setup_test_mat(ROWS, COLS);

    // our things
    let text: String = lowres(&frame, COLS, ROWS).expect("can't unwrap returned string");
    let re = Regex::new(
        r"\x1B\[48;2;(\d+);(\d+);(\d+)m ")
        .expect("regex fialed to build");

    for caps in re.captures_iter(&text) {
        let bg = (
            caps[1].parse::<u8>().unwrap(),
            caps[2].parse::<u8>().unwrap(),
            caps[3].parse::<u8>().unwrap(),
        );
        
        assert_eq!(bg, (0, 0 ,0)); // all black images
    }
}
