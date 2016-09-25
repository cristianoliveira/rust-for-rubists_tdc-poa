fn take(list: &Vec<i32>) {
    println!("first: {}", list[0]);
}

fn add_one_to(list: &Vec<i32>) {
    list.push(1);
    // Error cannot borrow immutable borrowed content `*list` as mutable
}

fn main() {
    let list = vec![10, 5, 4];
    take(&list); // ok
    take(&list); // ok
    add_one_to(&list);
}
