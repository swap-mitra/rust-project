struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user = User {
        first_name: String::from("Horidas"),
        last_name: String::from("Pal"),
        age: 32,
    };

    println!("{}", user.first_name);
    println!("{}", user.last_name);
    println!("{}", user.age);
}
