trait InDays {
  fn in_days(self) -> String;
}

impl InDays for i32 {
   fn in_days(self) -> String {
       if self > 1 {
           format!("{} days", self.to_string())
       } else {
           format!("{} day", self.to_string())
       }
   }
}

fn main() {
    println!("{}", 5.in_days());
    println!("{}", 1.in_days());
}
