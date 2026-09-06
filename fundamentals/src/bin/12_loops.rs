fn main() {
    let mut i = 3;
    loop {
        println!("{:?}", i);
        i -= 1;
        if i == 0 {
            break;
        }
    }
    println!("Done!");
    println!("");
    
    let mut my_num = 5;
    while my_num > 0 {
        println!("{my_num}");
        my_num -= 1;
    }
    println!("Done");
}
