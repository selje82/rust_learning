fn first_name(first_name: &str) {
    println!("{first_name}");
}

fn last_name(last_name: &str) {
    println!("{last_name}");
}

fn main() {
    let f_name = "Tom";
    let l_name = "Jones";

    first_name(f_name);
    last_name(l_name);

}