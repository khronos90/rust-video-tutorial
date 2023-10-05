pub enum ColorNames {
    // Red,
    // Green,
    Blue,
}

pub fn color_names(color: ColorNames) {
    match color {
        ColorNames::Blue => println!("blue"),
        // ColorNames::Green => println!("green"),
        // ColorNames::Red => println!("red"),
    }
}
