use std::fs::read_to_string;

fn main() {
    let result = read_to_string("src\\bin\\a.txt");

    match result {
        Ok(data) => println!("{}", data),
        Err(_) => println!("File not found!!"),
    }
}
