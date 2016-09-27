struct Foo;
trait Greeting {
    fn hi(&self) -> &str;
}

impl Greeting for Foo {
    fn hi(&self) -> &str {
        "foo"
    }
}

fn main() {
    let f = Foo;
    println!("{}", f.hi());
}

// Running
// cd examples/rust/
// rustc generics.rs && ./generics
