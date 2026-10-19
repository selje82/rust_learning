// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:

// * Use a struct containing the student's name and locker assignment
struct LockerAssign { 
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32>,
}
fn main() {
    let students = vec![
        LockerAssign {
            name: String::from("Garry"),
            locker: Some(2),
        },
        LockerAssign {
            name: String::from("Mary"),
            locker: None,
        },
        LockerAssign {
            name: String::from("Phil"),
            locker: Some(4),
        }
    ];
    
    for student in students {
        match student.locker {
            Some(locker) => println!("Student name: {} Locker: {}", student.name, locker),
            None => println!("Student: {} has not been assigned a locker yet.", student.name),
        }
    }
}
