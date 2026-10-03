fn main() {
    struct Pallet {
        length: f64,
        width: f64,
        height: f64,
    }

    let low_euro_pallet = Pallet {
        length: 1.2,
        width: 0.8,
        height: 1.2,
    };

    println!("The low pallet is {} meters long, has a width of {} meters, and are {} meters tall.",
    low_euro_pallet.length,
    low_euro_pallet.width,
    low_euro_pallet.height);
}
