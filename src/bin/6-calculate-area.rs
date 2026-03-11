enum Shape {
    Rectangle(f64, f64),
    Sqaure(f64),
    Circle(f64),
}

fn main() {
    let rect = Shape::Rectangle(25.0, 50.0);
    println!("Reactangle area: {}", calcute_area(rect));
    let sq = Shape::Sqaure(36.0);
    println!("Square area: {}", calcute_area(sq));
    let cir = Shape::Circle(31.47);
    println!("Circle area: {}", calcute_area(cir));
}

fn calcute_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(l, b) => l * b,
        Shape::Sqaure(s) => s * s,
        Shape::Circle(r) => 3.14 * r * r,
    }
}
