fn main() {
    let a = vec![10, 5, 4];
    let b = &a; // Empresta
    let c = &b; // Empresta

    println!("{:?}", a); // Ok
    println!("{:?}", b); // Ok
    println!("{:?}", c); // Ok
}

// Running
// cd examples/rust/
// rustc borrowing.rs && ./borrowing
