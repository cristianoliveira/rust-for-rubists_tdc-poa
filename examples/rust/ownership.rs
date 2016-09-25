fn take(list: Vec<i32>) {
    println!("first: {}", list[0]);
}

fn main() {
    let list = vec![10, 5, 4];
    let listb = list;
    let listc = listb;

    println!("{:?}", list); // ERRROUUU <Fausto Silva Voice>
    println!("{:?}", listb);
    // println!("{:?}", listc);
    //
    // take(list); // ok
    // take(list); // Error use of moved value: `list`
}

// Running
// cd examples/rust/
// rustc ownership.rs && ./ownership
