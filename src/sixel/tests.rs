use super::*;

#[test]
fn test_rgb_to_sixel_percent() {
    let a: u8 = rgb_to_sixel_percent(0);
    let b: u8 = rgb_to_sixel_percent(127);
    let c: u8 = rgb_to_sixel_percent(255);
    assert!(a == 0);
    assert!(b == 50);
    assert!(c == 100);
}

#[test]
fn test_calculate_sixel_cols() {
    let sixel: char = calculate_sixel_cols([true, false, true, false, true, false]);
    assert!(sixel == 'T');  // apparently the correct result is 'T'

    let sixel: char = calculate_sixel_cols([false, true, false, true, false, true]);
    assert!(sixel == 'i');  // correct result is 'i'
}

#[test]
fn test_sixel_is_on_bw() {
    let scenarios = [
        // basic cases
        ([255, 255, 255], 127, true),
        ([000, 000, 000], 127, false),

        // edge cases
        ([127, 127, 127], 127, false),  // equality
        ([255, 255, 255], 255, false),  // other more different equality
    ];
    for scenario in scenarios {
        let is_on = sixel_is_on_bw(Vec3b::from(scenario.0), scenario.1);
        assert!(is_on == scenario.2);
    }
}

#[test]
fn test_draw_sixel_colour() {
    let scenarios = [
        // basic cases
        ([255, 255, 255], 1, "#1;2;100;100;100#1"),  // colour #1
        ([000, 000, 000], 1, "#1;2;0;0;0#1"),

        ([127, 127, 127], 69, "#69;2;50;50;50#69"),  // colour #69
        ([000, 000, 000], 69, "#69;2;0;0;0#69"),

        ([255, 255, 255], 0, "#0;2;100;100;100#0"),  // colour #0
        ([127, 127, 127], 0, "#0;2;50;50;50#0"),
    ];
    for scenario in scenarios {
        let output: String = draw_sixel_colour(Vec3b::from(scenario.0), scenario.1);
        assert!(output == scenario.2);
    }
}