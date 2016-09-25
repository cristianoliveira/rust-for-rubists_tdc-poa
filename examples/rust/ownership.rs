fn take(list: Vec<i32>) {
    println!("first: {}", list[0]);
}

fn main() {
    let list = vec![10, 5, 4];
    take(list); // ok
    take(list); // Error use of moved value: `list`
}
