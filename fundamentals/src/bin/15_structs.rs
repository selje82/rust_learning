fn main() {
    struct Pallet {
        length: f32,
        width: f32,
        height: f32,
    }

    let low_euro_pallet = Pallet {
        length: 1.2,
        width: 0.8,
        height: 1.2,
    };

    let euro_length = low_euro_pallet.length;
    let euro_width = low_euro_pallet.width;
    let euro_height = low_euro_pallet.height;
    println!("The low pallet is {euro_length} meters long, has a width of {euro_width} meters, and are {euro_height} meters tall.");
}
