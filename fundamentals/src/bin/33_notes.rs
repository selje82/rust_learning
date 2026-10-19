fn main() {
    let numbers = vec![1, 2, 3];

    for num in &numbers {
        match numbers.is_empty() {
            true => println!("No numbers"),
            false => println!("Number {}", num),
        }
    }
}