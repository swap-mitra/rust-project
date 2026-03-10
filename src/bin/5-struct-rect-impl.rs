struct Rect {
    length: u32,
    breadth: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.breadth * self.length
    }

    fn perimeter(&self) -> u32 {
        2 * (self.breadth + self.length)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        length: 10,
        breadth: 20,
    };

    println!("Area is {}", rect1.area());
    println!("Perimeter is {}", rect1.perimeter());
    println!("Debug {}", Rect::debug());
}
