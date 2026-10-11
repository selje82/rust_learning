#[derive(Debug)]
enum Genre {
    Action,
    Comedy,
}
#[derive(Debug)]
struct Movie {
    title: String, 
    genre: Genre,
}

impl Movie {
    fn print(&self) {
        println!("Title: {}", &self.title);
        match self.genre {
            Genre::Action => println!("Action"),
            Genre::Comedy => println!("Comedy"),
        }
    }
}

fn main() {
    let my_movie = Movie {
        title: String::from("Last Rainbow"),
        genre: Genre::Action,
    };
    println!("{:#?}", my_movie);
}