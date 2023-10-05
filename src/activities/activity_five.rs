pub fn looping() {
    let mut var = 0;
    loop {
        var = var + 1;
        if var > 4 {
            break;
        }
        println!("{:?}", var);
    }
    println!("Done!");
}
