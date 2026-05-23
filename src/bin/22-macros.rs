use std::fmt::{ Debug, Display };

#[derive(Debug)]
struct User {
    username: String,
    age: u32,
}

//trait
impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "This the user struct with age {}", &self.age)
    }
}

//trait
// impl Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "This the user struct with age {}", &self.age)
//     }
// }

fn main() {
    let u = User {
        username: String::from("Elliot"),
        age: 22,
    };

    println!("{}", u);
    println!("{:?}", u);
    println!("{}", u.username)
}
