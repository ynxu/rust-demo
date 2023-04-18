
#![allow(unused)]
use std::cell::Cell;
fn main() {
    println!("111");
  let c = Cell::new("asdf");
  let one = c.get();
  c.set("qwer");
  let two = c.get();
  println!("111- {},{}", one, two);
}