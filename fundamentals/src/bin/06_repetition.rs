fn main() {
    // Setting num1 as int with value of 0.
    let mut num1 = 0;

    // loop until num1 is eq to 5. 
    loop {
        if num1 == 5 {
            break;
        } 
        // Displays the number after each iteration.
        println!("{num1:?}");
        num1 += 1;
    }

    // Checking that variable num1 is updated after the loop.
    println!();
    println!("New value of num1: {num1}");
    println!();

    // Setting num2 as int with value of 0.
    let mut num2 = 0;

    // While loop when num2 is not equal to 5. 
    while num2 != 5 {
        // Printing num2 value after each iteration.
        println!("{num2}");
        num2 += 1;
    }

    // Checking that variable num2 is updated after the while loop. 
    println!();
    println!("New value of num2: {num2}");
    println!();
}