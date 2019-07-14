use image::{DynamicImage, RgbImage};
use std::io::Write;

fn main() {
    let term = crossterm::Crossterm::new();
    let terminal = term.terminal();
    let color = term.color();
    //let input = term.input().read_sync();

    let im: DynamicImage = image::open("./ferris.png").unwrap();
    let im = im.to_rgb();

    let ascii = rascii::run(im, (20, 20), true, 8).unwrap();

    let mut stdout = std::io::stdout();
    let color_fg = |_w: &mut std::io::Stdout, (r, g, b): (u8, u8, u8)| {
        color.set_fg(crossterm::Color::Rgb { r, g, b });
    };

    let color_bg = |_w: &mut std::io::Stdout, (r, g, b): (u8, u8, u8)| {
        color.set_bg(crossterm::Color::Rgb { r, g, b });
    };

    rascii::print(ascii, &mut stdout, Some(color_fg), Some(color_bg));

    //println!("{}", stdout);
}
