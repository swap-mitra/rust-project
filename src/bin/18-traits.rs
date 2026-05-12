pub trait Summary {
    fn summarize(&self) -> String {
        //default implementation
        return String::from("Summarized");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("The name is {} and the age is {}", self.name, self.age);
    }
}

fn notify(item: impl Summary) {
    println!("Notification: {}", item.summarize())
}

fn main() {
    let user = User {
        name: String::from("Rukmini"),
        age: 28,
    };
    notify(user);
}
