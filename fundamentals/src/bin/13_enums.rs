#[allow(dead_code)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let go = Direction::Left;
    match go {
        Direction::Left => println!("Go left"),
        Direction::Right => println!("Go right"),
    }
}