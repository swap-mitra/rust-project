use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();

    users.insert(String::from("rukmini"), 24);
    users.insert(String::from("kalyani"), 21);
    users.insert(String::from("wamiqa"), 28);

    let user_age = users.get("kalyani");

    match user_age {
        Some(age) => println!("Age is {}", age),
        None => println!("user not found in db"),
    }
}
