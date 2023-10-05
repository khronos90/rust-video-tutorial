struct Person {
    age: i32,
    name: String,
    colour: String,
}

fn print_string(val: &str) {
    println!("{}", val);
}

pub fn strings() {
    let persons = vec![
        Person {
            age: 33,
            name: "Bruno".to_owned(),
            colour: String::from("Green"),
        },
        Person {
            age: 4,
            name: String::from("Violeta"),
            colour: String::from("Violeta"),
        },
    ];

    for person in persons {
        if person.age < 10 {
            println!("{:?}", person.age);
            print_string(&person.name);
            print_string(&person.colour);
        }
    }
}
