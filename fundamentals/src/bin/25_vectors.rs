struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test {score: 90},
        Test {score: 75},
        Test {score: 77},
        Test {score: 89},
    ];

    for test in my_scores {
        println!("Score: {}", test.score);
    }
}