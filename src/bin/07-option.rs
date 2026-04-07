fn main() {
    let index = find_first_a(String::from("rukmini"));

    match index {
        Some(value) => print!("Index at {}", value),
        None => println!("Not Found!!"),
    }
}

fn find_first_a(s: String) -> Option<u32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as u32);
        }
    }

    return None;
}
