struct Bar;
struct Foo;

trait Greeting {
    fn hi(&self) -> &str;
}

impl Greeting for Foo {
    fn hi(&self) -> &str {
        "foo"
    }
}

impl Greeting for Bar {
    fn hi(&self) -> &str {
        "bar"
    }
}

fn hello<T: Greeting>(f: T) {
    println!("{}", f.hi());
}

fn main() {
    hello(Bar);
    hello(Foo);
}

// Running
// cd examples/rust/
// rustc generics.rs && ./generics
