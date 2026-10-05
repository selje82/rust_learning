// Initializing enum for CargoType.
#[allow(dead_code)]
enum CargoType {
    Standard,
    Fragile,
    Hazardous,
}

impl CargoType {
    fn print(&self) {
        match self {
            Self::Standard => println!("Standard"),
            Self::Fragile => println!("Fragile"),
            Self::Hazardous => println!("Hazardous"), 
        }
    }
}

struct Dimensions {
    length: f64,
    width: f64,
    height: f64,
}

impl Dimensions {
    fn new(length: f64, width: f64, height: f64) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    fn print(&self) {
        println!("Length: {}, Width: {}, Height: {}", self.length, self.width, self.height);
    }
}

struct CargoPallet {
    id: i32,
    weight_kg: f64,
    dimensions: Dimensions,
    cargo_type: CargoType,
}

impl CargoPallet {
    fn new(id: i32, weight_kg: f64, dimensions: Dimensions, cargo_type: CargoType) -> Self {
        Self {
            id,
            weight_kg,
            dimensions,
            cargo_type,
        }
    }

    fn print(&self) {
        println!("ID: {}, Weight: {}", self.id, self.weight_kg);
        self.dimensions.print();
        self.cargo_type.print();
    }
}

fn main() {
    let my_dim = Dimensions::new(2.0, 3.4, 1.2);
    let my_pallet = CargoPallet::new(1, 12.3, my_dim, CargoType::Fragile);
    my_pallet.print()
}
