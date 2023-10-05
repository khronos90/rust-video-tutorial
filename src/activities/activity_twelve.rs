enum Color {
    // Red,
    Blue,
    // Yellow,
}

struct Box {
    dimensions: (i32, i32, i32),
    weight: f64,
    color: Color,
}

impl Box {
    fn new(color: Color, dimensions: (i32, i32, i32), weight: f64) -> Self {
        Self {
            color,
            dimensions,
            weight,
        }
    }

    fn print_box(&self) {
        let color_print = match self.color {
            Color::Blue => "Blue",
            // Color::Red => "Red",
            // Color::Yellow => "Yellow",
        };
        println!(
            "Dimensions {:?} Weight {:?} Color {:?}",
            self.dimensions, self.weight, color_print
        );
    }
}

pub fn shipping_box() {
    println!("Activity 12");
    let my_box = Box::new(Color::Blue, (10, 23, 1), 23.0);
    my_box.print_box();
}
