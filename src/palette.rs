macro_rules! palette_from_hexcodes {
    ( $( $hex:expr ),* $(,)? ) => {{
        vec![
            $(
                (
                    (($hex >> 16) & 0xFF) as u8,
                    (($hex >> 8) & 0xFF) as u8,
                    ($hex & 0xFF) as u8,
                )
            ),*
        ]
    }};
}

/// Microsoft VGA 16-colour palette
pub fn sixteen_dated() -> Vec<(u8,u8,u8)> {
    return vec![
        (000,000,000),  // black

        // some colours
        (127,000,000),  // dark red
        (255,000,000),  // bright red
        (000,127,000),  // dark green
        (000,255,000),  // bright green
        (000,000,127),  // dark blue
        (000,000,255),  // bright blue

        // colour combinations
        (127,000,127),  // dark purple
        (255,000,255),  // light puruple

        (127,127,000),  // dark orange
        (255,255,000),  // bright orange

        (000,127,127),  // cyan
        (000,255,255),  // cyan

        (064, 064, 064), // dark gray
        (128, 128, 128), // medium gray
        (192, 192, 192), // light gray (keeping one light gray for highlights)

        (255,255,255),   // white
    ]
}

pub fn sixteen() -> Vec<(u8, u8, u8)> {
    palette_from_hexcodes!(
        0x000000,
        0x800000,
        0x008000,
        0x808000,
        0x000080,
        0x800080,
        0x008080,
        0xc0c0c0,
        0x808080,
        0xff0000,
        0x00ff00,
        0xffff00,
        0x0000ff,
        0xff00ff,
        0x00ffff,
        0xffffff,
    )
}

pub fn p_6x6x6_rgb_cube() -> Vec<(u8,u8,u8)> {
    let mut palette = Vec::new();
    let steps = [0, 51, 102, 153, 204, 255];

    for &r in &steps {
        for &g in &steps {
            for &b in &steps {
                palette.push((r, g, b));
            }
        }
    }

    return palette
}

pub fn sixtyfour() -> Vec<(u8,u8,u8)> {
    let mut palette = Vec::new();
    let steps = [0, 85, 170, 255];

    for &r in &steps {
        for &g in &steps {
            for &b in &steps {
                palette.push((r, g, b));
            }
        }
    }

    palette
}

/// This palette separates colours into the following categories:
/// - dark
/// - medium
/// - pure 
/// 
/// This strict categorization allows it to cover many different colours.
/// Making it a generally ideal palette for videos and images.
/// 
/// It generally produces extremely washed out images
pub fn manyshades() -> Vec<(u8,u8,u8)> {
    return vec![
        // basic colors
        (0, 0, 0),     // pure black
        (255, 255, 255), // pure white

        // primary axis
        (64, 0, 0),    // dark red
        (128, 0, 0),   // medium red
        (255, 0, 0),   // pure red

        (0, 64, 0),    // dark green
        (0, 128, 0),   // medium green
        (0, 255, 0),   // pure green

        (0, 0, 64),    // dark blue
        (0, 0, 128),   // medium blue
        (0, 0, 255),   // pure blue

        // secondary axis
        (0, 64, 64),   // dark cyan
        (0, 128, 128), // medium cyan
        (0, 255, 255), // pure cyan

        (64, 0, 64),   // dark magenta
        (128, 0, 128), // medium magenta
        (255, 0, 255), // pure magenta

        (64, 64, 0),   // dark yellow
        (128, 128, 0), // medium yellow
        (255, 255, 0), // pure yellow

        (255, 140, 000), // dark orange
        (236, 172, 118), // mid orange
        (255, 128, 000), // pure orange

        // neutrals
        (64, 64, 64),   // dark gray
        (128, 128, 128), // medium gray
        (192, 192, 192), // light gray (keeping one light gray for highlights)
    ]
}

pub fn general() -> Vec<(u8, u8, u8)> {
    return vec![
        (000, 000, 000),  // black
        (255, 255, 255),  // white

        (128, 000, 000),  // red
        (255, 000, 000),  // red again

        (000, 128, 000),  // green
        (000, 255, 000),  // green again

        (128, 192, 255),  // blue but lighter
        (000, 255, 000),  // blue again

        (255, 128, 000),  // orange
        (000, 192, 192),  // miku hair colour
        (127, 000, 127),  // purple
        (127, 064, 000),  // brown
        (255, 000, 255),  // magenta
        (255, 255, 000),  // yellow
    ]
}