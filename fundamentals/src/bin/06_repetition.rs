fn main() {
    let mut num1 = 0;

    loop {
        if num1 == 5 {
            break;
        } 
        println!("{num1:?}");
        num1 += 1;
    }
    println!();
    println!("New value of num1: {num1}");
    println!();

    let mut num2 = 0;
    while num2 != 5 {
        println!("{num2}");
        num2 += 1;
    }

    println!();
    println!("New value of num2: {num2}");
    println!();
}