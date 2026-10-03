#[allow(dead_code)]
// Setting access level for different user classes. 
enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main() {
    // secret file: admins only
    let access_level = Access::User;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false, 
    };
    println!("Can access: {can_access_file}");
}