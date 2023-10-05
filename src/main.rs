mod activities {
    pub mod activite_four;
    pub mod activity_13;
    pub mod activity_14;
    pub mod activity_15;
    pub mod activity_16;
    pub mod activity_eigth;
    pub mod activity_eleven;
    pub mod activity_five;
    pub mod activity_nine;
    pub mod activity_one;
    pub mod activity_seven;
    pub mod activity_six;
    pub mod activity_ten;
    pub mod activity_three;
    pub mod activity_twelve;
    pub mod activity_two;
}

fn main() {
    println!("Hello, world!");

    let my_var = 42;

    // pascalCase vars break the compiler
    println!("My var: {:?}", my_var);

    // Control structures
    let a = 99;
    if a > 99 {
        // println!("Big munber");
    } else {
        // println!("Small number");
    }

    // Repetition
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        // println!("{:?}", a);
        a += 1;
    }

    let mut b = 0;
    while b != 5 {
        // println!("{:?}", b);
        b = b + 1;
    }
    activities::activity_one::print_name();

    // Arithmetic
    // let sum = 2 + 2;
    // let substraction = 10 - 5;
    // let division = 10 / 2;
    // let multiplication = 5 * 5;

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }

    let sub_result = sub(8, 3);
    println!("{sub_result}");

    // let rem = 6 % 3; // 0
    // let rem2 = 6 % 4; // 2

    activities::activity_two::basic_arithmetic(10, 23);
    println!("### Control flow");
    activities::activity_three::control_flow();
    activities::activity_three::control_flow_two(5);

    let some_bool = true;

    match some_bool {
        true => println!("Yeah true"),
        false => println!("Nay false"),
    }

    let some_int = 3;

    match some_int {
        1 => println!("It's 1"),
        3 => println!("It's 3"),
        _ => println!("It's something else"), // Exhaustive check on all integers
    }

    let my_name = "Bruno";

    match my_name {
        "Bruno" => println!("My name!"),
        "Gonza" => println!("Not my name!"),
        "Alice" => println!("Hello Alice"),
        _ => println!("Nice to meet you!"),
    };

    activities::activite_four::decision_making(false);
    activities::activite_four::decision_making_two(4);

    // Repetition with loop
    println!("### Loops");
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i = i - 1;
        if i == 0 {
            break;
        }
    }
    println!("Done!");

    activities::activity_five::looping();

    // Repetition with while
    println!("### While");
    activities::activity_six::count_while();

    // Enums
    println!("### Enums");
    enum Direction {
        Down,
    }

    let go_up = Direction::Down;

    match go_up {
        Direction::Down => println!("Up!"),
        _ => println!("Not up!"),
    };

    activities::activity_seven::color_names(activities::activity_seven::ColorNames::Blue);

    // structs
    println!("### Structs");
    struct ShippingBox {
        depth: i32,
        width: i32,
        height: i32,
    }

    let my_box = ShippingBox {
        depth: 3,
        height: 2,
        width: 5,
    };

    let tall = my_box.height;
    println!("The box is {:?} units tall", tall);

    activities::activity_eigth::drink();

    // tuples
    println!("### Tuples");
    activities::activity_nine::destructur();

    // expressions
    println!("### Expressions");

    activities::activity_ten::expressions();

    // Basic memory
    activities::activity_eleven::ownership();

    // impl (i m p l)
    struct Temperature {
        degrees_f: f64,
    }

    impl Temperature {
        // Self capitalized refers to Temperature itself
        fn freezing() -> Self {
            Self { degrees_f: 32.0 }
        }

        fn boiling() -> Self {
            Self { degrees_f: 212.0 }
        }

        fn show_temp(&self) {
            println!("{:?} degrees F", self.degrees_f);
        }
    }

    let hot = Temperature { degrees_f: 99.9 };
    hot.show_temp();
    let cold = Temperature::freezing();
    cold.show_temp();
    let boiling = Temperature::boiling();
    boiling.show_temp();

    activities::activity_twelve::shipping_box();

    // vectors
    println!("### Vectors");

    // Data of the same type
    let my_numbers = vec![1, 2, 3];
    // or
    // vec! expands to something similar to this below
    let mut my_numbers = Vec::new();
    my_numbers.push(1);
    my_numbers.push(2);
    my_numbers.push(3);
    my_numbers.pop();
    my_numbers.len();
    let two = my_numbers[1];

    for num in my_numbers {
        println!("{:?}", num);
    }

    activities::activity_13::vectors();

    // String &str
    activities::activity_14::strings();

    // Derive macro
    // #[derive(Debug)]
    // Special macro that is applied to enums and structs
    // ALlows you to automatically implement some functionality
    // With debug we can print data directly from an enum for ie.

    #[derive(Debug)]
    enum Position {
        Manager,
        Supervisor,
        Worker,
    }

    #[derive(Debug)]
    struct Employee {
        position: Position,
        work_hours: i64,
    }

    let employee = Employee {
        position: Position::Supervisor,
        work_hours: 48,
    };

    println!("{:?}", employee);

    // advanced match
    activities::activity_15::advanced_match();

    // option type
    println!("### Option type");
    /*
        enum Option<T> {
           Some<T>,
           None
        }
    */

    struct Customer {
        age: Option<i32>,
        email: String,
    }

    let mark = Customer {
        age: Some(32),
        email: "mark@email.com".to_owned(),
    };

    let becky = Customer {
        age: None,
        email: "beck@email.com".to_owned(),
    };

    match becky.age {
        Some(age) => println!("customer is {:?} years old", age),
        None => println!("customer age not provided"),
    }

    struct GroceryItem {
        name: String,
        qty: i32,
    }

    fn find_quantity(name: &str) -> Option<i32> {
        let groceries = vec![
            GroceryItem {
                name: "bananas".to_owned(),
                qty: 4,
            },
            GroceryItem {
                name: "eggs".to_owned(),
                qty: 12,
            },
            GroceryItem {
                name: "bread".to_owned(),
                qty: 1,
            },
        ];
        for item in groceries {
            if item.name == name {
                return Some(item.qty);
            }
        }
        None
    }
    let item = "cucurucho";
    match find_quantity(item) {
        Some(qty) => println!("Item {:?} has qty {:?}", item, qty),
        None => println!("Item {:?} has no qty", item),
    }

    // Standard library usage
    activities::activity_16::standard_lib();
}
