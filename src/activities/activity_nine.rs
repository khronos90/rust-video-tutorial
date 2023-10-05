fn my_tuple() -> (i32, i32) {
    (6, 4)
}

pub fn destructur() {
    let (x, y) = my_tuple();
    println!("{:}", x);
    if y > 5 {
        println!("> 5")
    } else if y < 5 {
        println!("< 5")
    } else {
        println!("5")
    }
}
