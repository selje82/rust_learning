#[derive(Debug)]
#[allow(dead_code)]
enum Genre {
    Action,
    Comedy,
}
#[derive(Debug)]
#[allow(dead_code)]
struct Movie {
    title: String, 
    genre: Genre,
}

fn main() {
    let my_movie = Movie {
        title: String::from("Last Rainbow"),
        genre: Genre::Action,
    };
    println!("{:#?}", my_movie);
}