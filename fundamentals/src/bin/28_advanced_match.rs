#[allow(dead_code)]
enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let num = 3;
    match num {
        3 => println!("3"),
        _ => println!("Number: {}", num),
    }

    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        Discount::Flat(amount) => println!("Flat discount of {}", amount),
        _ => (),
    };

    let concert = Ticket {
        event: String::from("concert"),
        price: 50,
    };

    match concert {
        Ticket{price: 50, event} => println!("Event @ 50 = {}", event),
        Ticket{price, ..} => println!("Price = {}", price),
    };
}