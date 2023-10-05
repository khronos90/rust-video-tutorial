pub fn expressions() {
    let var = 120;
    let its_bt_100 = if var > 100 { true } else { false };
    match its_bt_100 {
        true => println!(">100"),
        false => println!("<=100"),
    }
}
