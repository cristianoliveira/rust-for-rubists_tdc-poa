use std::thread;

fn main() {
    let words = vec!["Hello", "World", "From", "Rust"];
    words.push(" lol ");

    let threads = for w in words.clone() {
        let _ = thread::spawn(move ||{
            println!("{}", w);
        });
    };
    threads.join()
}

// Running
// cd examples/rust/
// rustc race_condition.rs && ./race_condition
