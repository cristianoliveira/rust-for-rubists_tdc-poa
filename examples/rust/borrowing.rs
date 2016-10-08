fn take(list: &Vec<i32>) { println!("{:?}", list)}

fn main() {
    let a = vec![10, 5, 4];
    take(&a); // Empresta
    println!("{:?}", a); // ok
}

// Running
// cd examples/rust/
// rustc borrowing.rs && ./borrowing
