fn main() {
    let term = crossterm::Crossterm::new();
    let color = term.color();

    let ascii = rascii::image_to_ascii("./ferris.png", (20, 20), true, 0).unwrap();

    let mut stdout = std::io::stdout();
    let color_fg = |_w: &mut std::io::Stdout, (r, g, b): (u8, u8, u8)| {
        let _ = color.set_fg(crossterm::Color::Rgb { r, g, b });
    };

    let color_bg = |_w: &mut std::io::Stdout, (r, g, b): (u8, u8, u8)| {
        let _ = color.set_bg(crossterm::Color::Rgb { r, g, b });
    };

    let _ = rascii::print_ascii(ascii, &mut stdout, Some(color_fg), Some(color_bg));
}
