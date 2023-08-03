use std::collections::{HashMap};
use std::time::Instant;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Position {
  x: i32,
}

fn main() {
  let mut result = 0;
  let start = Instant::now();
  for x in 0..10_000 {
    let mut map: HashMap<Position,usize> = HashMap::with_capacity(10_000);
    for y in 0..10_000 {
      map.insert(Position{x: x * 10_000 + y}, 0);
    }
    result += map.len();
  }
  println!("Time elapsed is: {:?} = {result}", start.elapsed());
}
