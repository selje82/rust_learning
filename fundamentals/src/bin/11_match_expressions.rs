fn main() {
    let some_bool = false;
    match some_bool {
        true => println!("It's true"),
        false => println!("It's false")
    }

    let some_num = 4;
    match some_num {
        1 => println!("It's 1"),
        2 => println!("It's 2"),
        3 => println!("It's 3"),
        4 => println!("It's 4"),
        _ => println!("Higher than 4.")
    }

    let my_name = "Rune Nyhrer";
    match my_name {
        "Regnell A" => println!("Ikke like god på jug."),
        "Høvdingen" => println!("Ikke så sleten..."),
        "Skink" => println!("CIP CIP"),
        "Thomas" => println!("Jobber ikke med GLN..."),
        _ => println!("Kan det være noen andre fra Pekeleke?"),
    }
}