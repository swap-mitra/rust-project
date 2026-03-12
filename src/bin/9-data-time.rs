use chrono::{ Local, Utc };
fn main() {
    let utc = Utc::now();
    println!("Current UTC time is {}", utc);

    let local = Local::now();
    println!("Current local time is {}", local);
}
