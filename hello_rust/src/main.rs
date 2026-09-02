fn main() {
    let num1 = 4;
    let num2 = 7;

    let x = add(num1, num2);
    println!("The sum of {} and {} is: {}", num1, num2, x);

}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
