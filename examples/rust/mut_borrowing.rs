fn take(list: &Vec<i32>) {
    println!("first: {}", list[0]);
}

fn add_one_to(list: &mut Vec<i32>) {
    list.push(1);
}

fn main() {
    let mut list = vec![10, 5, 4];
    // let mut borrowed = &mut list;
    take(&list); // ok
    take(&list); // ok
    add_one_to(&mut list);
    println!("{:?}", list); // ok
}

// Running
// cd examples/rust/
// rustc mut_borrowing.rs && ./mut_borrowing
