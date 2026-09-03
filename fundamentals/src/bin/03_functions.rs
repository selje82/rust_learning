fn main() {
    let num1 = 4;
    let num2 = 3;

    let x = add(num1, num2);
    println!("The sum of {num1:?} and {num2:?} is {x:?}.");

}

fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}