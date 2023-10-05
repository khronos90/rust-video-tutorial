pub fn vectors() {
    let the_numbers = vec![10, 20, 30, 40];

    // The for loop has to borrow
    for num in &the_numbers {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}", num),
        }
    }
    println!("The length is {:?}", the_numbers.len());
}
