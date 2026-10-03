fn main() {
    let mut i = 3;
    loop {
        println!("{:}", i);
        i -= 1;
        if i == 0 {
            break;
        }
    }
    println!("Done!");

    let mut num = 3;
    while num > 0 {
        println!("{:}", num);
        num -= 1;
    }
}
