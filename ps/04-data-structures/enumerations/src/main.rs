enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    CmykColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, //struct
}

fn main() {
    //let c: Color = Color::Blue;
    //let c: Color = Color::RgbColor(0, 0, 0);
    //let c: Color = Color::RgbColor(1, 2, 3);
    let c: Color = Color::CmykColor {
        cyan: 0,
        magenta: 128,
        yellow: 0,
        black: 255,
    };

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        //_ => println!("some other color"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0)
        | Color::CmykColor {
            cyan: _,
            magenta: _,
            yellow: _,
            black: 255,
        } => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        _ => (),
    }
}
