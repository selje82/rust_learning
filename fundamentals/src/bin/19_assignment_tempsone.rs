#[allow(dead_code)]
enum TemperaturZone {
    Dry,
    Cool,
    Frozen,
}
#[allow(dead_code)]
struct Order {
    order_id: i32,
    zone: TemperaturZone,
    weight_kg: f64,
}

fn calculate_fee(order: Order) -> (bool, f64) {
    let mut total_price = 0.00;
    let mut valid = true;
    match order.zone {
        TemperaturZone::Dry => total_price += 100.0,
        TemperaturZone::Cool => total_price += 250.0,
        TemperaturZone::Frozen => total_price += 400.0,
    }
    if order.weight_kg > 1000.0  {
        total_price += 150.0
    }
    if order.weight_kg > 1500.0 {
        valid = false
    }

    (valid, total_price)
}

fn main() {
    let my_order = Order {
        order_id: 1,
        zone: TemperaturZone::Cool,
        weight_kg: 1200.0,
    };

    let (valid, fee) = calculate_fee(my_order);
    let mut counter = 1;

    while counter < 4 {
        println!("Scanning pallet {counter}");
        counter += 1;
    }
    println!("Scanning finished");
      if valid {
        println!("Total fee: ${fee}.")
    } else {
        println!("Sorry, the order is too heavy.")
    }
}