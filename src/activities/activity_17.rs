#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                name: name.to_string(),
                age,
            })
        } else {
            Err("Adult's are 21 or older")
        }
    }
}

pub fn results() {
    let adult = Adult::new("Bruno", 23);
    let child = Adult::new("Anita", 15);

    match adult {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{:?}", e),
    }
    match child {
        Ok(a) => println!("{:?}", a),
        Err(e) => println!("{:?}", e),
    }
}
