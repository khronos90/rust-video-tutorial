pub fn control_flow() {
    let flag = true;
    if flag {
        println!("Hello")
    } else {
        println!("goodbye")
    }
}

pub fn control_flow_two(var: i32) {
    if var < 5 {
        println!("Less than 5");
    } else if var == 5 {
        println!("Just 5");
    } else {
        println!("More than 5");
    }
}