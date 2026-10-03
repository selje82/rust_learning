#[allow(dead_code)]

// Setting up different types of pallets. 
enum PalletType {
    Euro,
    Half,
    NLP,
}

// Using Pallettype as pallet_type and adding a field for weight in kg. 
struct Shipment {
    pallet_type: PalletType,
    weight_kg: f64,
}

// Function to check PalletType and print type and weight of pallet. 
fn print_shipment(shipment: Shipment) {
    //Check pallet type.
    match shipment.pallet_type {
        PalletType::Euro => println!("Pallet type: Euro"),
        PalletType::Half => println!("Pallet type: Half"),
        PalletType::NLP => println!("Pallet type: NLP"),
    }
    //Print weight of pallet.
    println!("Weight of pallet: {} kg.", shipment.weight_kg);
}

fn main() {
    // Making a pallet of type Euro with weight of 380.23kg. 
    let my_shipment = Shipment {
        pallet_type: PalletType::Euro,
        weight_kg: 380.23,
    };

    // Calling the function.
    print_shipment(my_shipment);


}

