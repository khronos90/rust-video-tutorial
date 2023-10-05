struct Grocery {
    id: i32,
    quantity: i32,
}

fn display_quantity(grocery: &Grocery) {
    println!("Quantity: {:?}", grocery.quantity);
}

fn display_id(grocery: &Grocery) {
    println!("Id: {:?}", grocery.id);
}

pub fn ownership() {
    let my_grocery = Grocery {
        id: 1,
        quantity: 23,
    };
    display_id(&my_grocery);
    display_quantity(&my_grocery);
}
