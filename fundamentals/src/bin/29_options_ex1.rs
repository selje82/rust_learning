#[allow(dead_code)]
struct Customer {
    age: Option<i32>,
    email: String,
}

struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let groceries = vec![
        GroceryItem {name: String::from("Banans"), qty: 10},
        GroceryItem {name: String::from("Pizza"), qty: 7},
        GroceryItem {name: String::from("Bread"), qty: 2},
    ];

    for item in groceries {
        if item.name == name {
            return Some(item.qty);
        }
    }
    None
}

fn main() {
    #[allow(unused)]
    let mark = Customer {
        age: Some(22), email: String::from("mark@example.com"),
    };

    let becky = Customer {
        age: None, email: String::from("becky@example.com"),
    };

    match becky.age {
        Some(age) => println!("Customer is {} years old", age),
        None => println!("Customer age not provided"),
    }

    let grocery = "Bread";

    let qty = find_quantity(grocery);

    println!("Qty: {:?}", qty);

}