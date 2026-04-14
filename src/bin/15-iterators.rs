fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // iterator adapter
    let v1_iter2 = v1_iter.map(|x| x * 2);

    for i in v1_iter2 {
        println!("{}", i);
    }

    // let sum: i32 = v1_iter.sum();
    // println!("Sum is {sum}");

    //iter_mut impl
    // for value in v1_iter {
    //     *value = *value * 2;
    //     println!("Got: {value}");
    // }

    println!("Array: {:?}", v1)
}
