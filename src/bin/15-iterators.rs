fn main() {
    let v1 = vec![1, 2, 3];
    let iter = v1.iter();

    // iterator adapter -> map
    // let iter2 = iter.map(|x| x * 2);

    // for i in iter2 {
    //     println!("{}", i);
    // }

    //iterator adapter -> filter
    let iter3 = iter.filter(|x| *x % 2 == 0);
    for i in iter3 {
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
