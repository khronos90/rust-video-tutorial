pub fn basic_arithmetic(a: i32, b: i32) {
    let sum = add_two(a, b);
    print_sum(sum);
}

fn add_two(a: i32, b: i32) -> i32 {
    a + b
}

fn print_sum(val: i32) {
    println!("{:?}", val)
}