#[allow(dead_code)]
struct Customer {
    age: Option<i32>,
    email: String,
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
}