enum Direction {
    Left,
    Right,
}

fn main() {
    let go = Direction::Right;
    match go {
        Direction::Left => println!("Go left"),
        Direction::Right => println!("Go right"),
    }
}