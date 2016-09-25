trait IsOdd {
  fn is_odd(self) -> bool;
}

impl IsOdd for i32 {
   fn is_odd(self) -> bool {
       self % 2 == 0
   }
}

fn main() {
    println!("5 is odd: {}", 5.is_odd());
    println!("2 is odd: {}", 2.is_odd());
}
