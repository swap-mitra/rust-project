fn main() {
    let mut v1 = vec![1, 2, 3];
    let v1_iter = v1.iter_mut();

    for value in v1_iter {
        *value = *value * 2;
        println!("Got: {value}");
    }

    println!("Array: {:?}", v1)
}
