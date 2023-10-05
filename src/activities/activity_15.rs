enum TicketType {
    Standard,
    Vip(String),
    Backstage(String),
}

impl TicketType {
    fn to_string(&self) -> &str {
        return match &self {
            TicketType::Standard => "Standard",
            TicketType::Vip(_) => "Vip",
            TicketType::Backstage(_) => "Backstage",
            _ => "",
        };
    }
}

struct Ticket {
    ticket_type: TicketType,
    price: f64,
}

pub fn advanced_match() {
    let tickets = vec![
        Ticket {
            ticket_type: TicketType::Standard,
            price: 50.0,
        },
        Ticket {
            ticket_type: TicketType::Vip("Bruno".to_owned()),
            price: 150.0,
        },
        Ticket {
            ticket_type: TicketType::Backstage(String::from("Candela")),
            price: 200.0,
        },
    ];

    for ticket in tickets {
        match ticket {
            Ticket { ticket_type, price } => {
                println!(
                    "Ticket type {:?} with price {:?}",
                    ticket_type.to_string(),
                    price
                )
            }
        }
    }
}
