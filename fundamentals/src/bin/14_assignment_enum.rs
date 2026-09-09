enum GearState {
    Up,
    Down,
}

fn check_gear(state: GearState) {
    match state {
        GearState::Up => println!("Wheels are up, we are ready for cruise."),
        GearState::Down => println!("Wheels are down, we are ready for landing."),
    }
}

fn main() {
    let current_gear = GearState::Up;
    check_gear(current_gear);
}