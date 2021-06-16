use coloriz::*;

pub fn colors(){

    let amer_pal = &[
        Color::Red,
        Color::White,
        Color::Blue,
    ];

    let rain_pal = &[
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
    ];

    let bright_pal = &[
        Color::BrightBlack,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];

    for color in amer_pal{
        print!("{}",color);
    }

    for color in rain_pal{
        print!("{}",color);
    }

    for color in bright_pal{
        print!("{}",color);
    }

    println!("");



}