pub fn decision_making(var: bool) {
    match var {
        false => println!("It's false"),
        true => println!("It's true"),
    }
}

pub fn decision_making_two(var: i32) {
    match var {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("Other"),
    }
}
