use image::RgbImage;
use std::error::Error;
use std::io::Write;

/// 10 Levels of grayscale
const GSCALE_10: &[char] = &[' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
const GSCALE_70: &str = " .\"`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
const GAMMA: f64 = 2.2;

type RasciiOutput = Vec<Vec<(char, RasciiColor)>>;

#[derive(Debug)]
pub enum RasciiColor {
    RGB(u8, u8, u8),
    Grayscale(u8),
}

impl RasciiColor {
    fn to_grayscale(&self) -> u8 {
        match self {
            RasciiColor::RGB(r, g, b) => {
                let rlin = (*r as f64).powf(GAMMA);
                let blin = (*b as f64).powf(GAMMA);
                let glin = (*g as f64).powf(GAMMA);

                let y = (0.2126 * rlin) + (0.7152 * glin) + (0.0722 * blin);

                let l = (116.0 * y.powf(1.0 / 3.0) - 16.0) as u8;
                l
            }
            RasciiColor::Grayscale(l) => *l,
        }
    }
}

/// Convert the image to rascii based on the settings provided
pub fn run(
    image: RgbImage,
    dim: (u32, u32),
    color: bool,
    depth: u8,
) -> Result<RasciiOutput, Box<dyn Error>> {
    let mut output: RasciiOutput = Vec::new();
    // Dimensions of image
    let (width, height) = image.dimensions();

    // Get tile dimensions
    let tile_w = width / dim.0 as u32;
    let tile_h = height / dim.1 as u32;

    // Convert image to image chunks based on dimensions.
    for ty in 1..dim.1 - 1 {
        let mut row_tiles = Vec::new();

        for tx in 1..dim.0 - 1 {
            let mut tile_pixel_data = Vec::with_capacity((tile_w * tile_h) as usize);
            // per tile
            for px in 0..tile_w {
                for py in 0..tile_h {
                    let pixel_data = image.get_pixel(px + (tx * tile_w), py + (ty * tile_h)).data;

                    let tile_color: RasciiColor;
                    if color {
                        tile_color = RasciiColor::RGB(pixel_data[0], pixel_data[1], pixel_data[2])
                    } else {
                        let y = RasciiColor::RGB(pixel_data[0], pixel_data[1], pixel_data[2])
                            .to_grayscale();
                        tile_color = RasciiColor::Grayscale(y as u8);
                    }

                    tile_pixel_data.push(tile_color);
                }
            }

            let avg: RasciiColor;
            let ascii_char: char;
            if color {
                avg = RasciiColor::RGB(
                    (tile_pixel_data.iter().fold(0usize, |sum, x| {
                        sum + match x {
                            RasciiColor::RGB(r, _, _) => *r as usize,
                            _ => 0,
                        }
                    }) / tile_pixel_data.len()) as u8,
                    (tile_pixel_data.iter().fold(0usize, |sum, x| {
                        sum + match x {
                            RasciiColor::RGB(_, g, _) => *g as usize,
                            _ => 0,
                        }
                    }) / tile_pixel_data.len()) as u8,
                    (tile_pixel_data.iter().fold(0usize, |sum, x| {
                        sum + match x {
                            RasciiColor::RGB(_, _, b) => *b as usize,
                            _ => 0,
                        }
                    }) / tile_pixel_data.len()) as u8,
                );
                if depth > 10 {
                    let index = (avg.to_grayscale() as f64 / 255.0) * 67.0;
                    let chars = GSCALE_70.chars().collect::<Vec<char>>();
                    ascii_char = chars[index as usize];
                } else {
                    let index = (avg.to_grayscale() as f64 / 255.0) * 9.0;
                    ascii_char = GSCALE_10[index as usize];
                }
            } else {
                avg = RasciiColor::Grayscale(
                    (tile_pixel_data.iter().fold(0usize, |sum, x| {
                        sum + match x {
                            RasciiColor::Grayscale(x) => *x as usize,
                            _ => 0,
                        }
                    }) as usize
                        / tile_pixel_data.len()) as u8,
                );
                let x = match avg {
                    RasciiColor::Grayscale(x) => x,
                    _ => 0,
                };
                if depth > 10 {
                    let index = (x as f64 / 255.0) * 67.0;
                    let chars = GSCALE_70.chars().collect::<Vec<char>>();
                    ascii_char = chars[index as usize];
                } else {
                    let index = (x as f64 / 255.0) * 9.0;
                    ascii_char = GSCALE_10[index as usize];
                }
            }

            row_tiles.push((ascii_char, avg));
        }

        output.push(row_tiles);
    }

    // Convert to grayscale or rgb and extract average colors of each chunk

    // Figure out background color and character to show

    Ok(output)
}

pub fn print<W: Write, F1: Fn(&mut W, (u8, u8, u8)) + Copy, F2: Fn(&mut W, (u8, u8, u8)) + Copy>(
    output: RasciiOutput,
    stdout: &mut W,
    color_fg: Option<F1>,
    color_bg: Option<F2>,
) -> std::io::Result<()> {
    for row in output {
        for col in row {
            if let Some(color_fg) = color_fg {
                let (r, g, b) = match col.1 {
                    RasciiColor::RGB(r, g, b) => (r, g, b),
                    _ => (0, 0, 0),
                };

                color_fg(stdout, (r, g, b));
                if let Some(color_bg) = color_bg {
                    color_bg(stdout, (r, g, b));
                }
            }
            write!(stdout, "{}", col.0)?;
        }
        writeln!(stdout, "")?;
    }
    Ok(())
}
