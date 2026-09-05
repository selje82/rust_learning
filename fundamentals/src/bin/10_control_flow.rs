fn main() {
    let age = 15;
    if age >= 21 {
        println!("Ok to purchase");
    } else {
        println!("Cannot purchase");
    }

    let num_check = 6;

    if num_check >5 {
        println!(">5");
    } else if num_check <5 {
        println!("<5");
    } else {
        println!("=5");
    }
}