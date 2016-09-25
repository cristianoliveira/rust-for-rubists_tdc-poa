struct Foo;

impl Foo {
    pub fn hi(&self) -> &str {
        "Hello"
    }
}

fn main() {
    let f = Foo;
    f.hi();
}
