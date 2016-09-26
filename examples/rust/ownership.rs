fn main() {
    let a = vec![10, 5, 4];
    let b = a; // Transfere o owner
    let c = b; // Transfere o owner

    println!("{:?}", a); // ERRROUUU (Fausto Silva Voice)
    println!("{:?}", c); // único valido
}

// Running
// cd examples/rust/
// rustc ownership.rs && ./ownership
