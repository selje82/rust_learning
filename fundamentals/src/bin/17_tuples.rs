fn main() {
    
    // Making a tuple of coordinates
    let coord = (2, 3);
    println!("X: {}, Y: {}", coord.0, coord.1);

    // The same coordinates added with variable names x and y.
    let (x, y) = (2, 3);
    println!("X: {x}, Y: {y}");

    // Tuple of name and age.
    let (name, age) = ("Skink", 99);
    println!("Name: {name}, Age: {age}");

}