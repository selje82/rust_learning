// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
// * Use a function to display the result
fn show(result: i32) {
    println!("Sum: {:?}", result);
}
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    show(add(12,10));
}
