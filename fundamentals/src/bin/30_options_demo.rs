struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>,
}

fn main() {
    let response = Survey {
        q1: None,
        q2: Some(true),
        q3: Some(String::from("A")),
    };

    match response.q1 {
        Some(ans) => println!("Q1: {}", ans),
        None => println!("Q1: No response"),
    }
    match response.q2 {
        Some(ans) => println!("Q2: {}", ans),
        None => println!("Q2: No response"),
    }
    match response.q3 {
        Some(ans) => println!("Q3: {}", ans),
        None => println!("Q3: No response"),
    }
}
