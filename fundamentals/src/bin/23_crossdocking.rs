#[allow(dead_code)]
enum Destination {
    Domestic,
    International, 
    Local,
}

impl Destination {
    fn print(&self) {
        match self {
            Self::Domestic => println!("Domestic"),
            Self::Local => println!("Local"),
            Self::International => println!("International")
        }
    }
}

struct PackageSize {
    height: f64,
    width: f64,
    length: f64,
    weight_kg: f64,
}

impl PackageSize {
    fn new(height: f64, width: f64, length: f64, weight_kg: f64) -> Self {
        Self {
            height,
            length,
            width,
            weight_kg,
        }
    }
    fn print(&self) {
        println!("Height: {}, Length: {}, Width: {}, Weight(kg) {}", self.height, self.length, self.width, self.weight_kg);
    }
}

struct Shipment {
    id: i32,
    destination: Destination,
    size: PackageSize,
}

impl Shipment {
    fn new(id: i32, destination: Destination, size: PackageSize) -> Self {
        Self {
            id,
            destination,
            size,
        }
    }
    fn print(&self) {
        println!("ID: {}", self.id);
        self.destination.print();
        self.size.print();
    }
}

fn main() {
    let package1 = PackageSize::new(2.3, 3.2, 4.1, 1.0);
    let my_shipment1 = Shipment::new(3, Destination::International, package1);
    my_shipment1.print();
    let package2 = PackageSize::new(1.2, 3.2, 0.8, 0.2);
    let my_shipment2 = Shipment::new(1, Destination::Local, package2);
    my_shipment2.print();
}
