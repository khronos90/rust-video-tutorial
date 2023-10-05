enum DrinkFlavour {
    Cola,
    // Fanta,
    // Sprite,
}

struct Drink {
    flavour: DrinkFlavour,
    ounces: i32,
}

pub fn drink() {
    let coca_cola = Drink {
        flavour: DrinkFlavour::Cola,
        ounces: 2,
    };
    println!("The drink has {:?} ounces", coca_cola.ounces);
    match coca_cola.flavour {
        DrinkFlavour::Cola => println!("The flavour is cola"),
        // DrinkFlavour::Fanta => println!("The flavour is fanta"),
        // DrinkFlavour::Sprite => println!("The flavour is sprinte"),
    }
}
