fn main() {
    let mut s1 = String::from("hawjawbawrawlaw");
    do_something(&mut s1);
    println!("{}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" by sukumar da");
    println!("{}", s2);
}
