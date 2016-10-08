fn take(list: Vec<i32>) { println!("{:?}", list)}

fn main() {
    let a = vec![10, 5, 4];
    take(a); // Transfere o owner
    println!("{:?}", a); // ERRROUUU error: use of moved value: `a`
}

// Running
// cd examples/rust/
// rustc ownership.rs && ./ownership
