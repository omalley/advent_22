use std::collections::{HashMap};
use std::time::Instant;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

fn everybody_move(x: i32) -> usize {
  let mut map: HashMap<Position,usize> = HashMap::with_capacity(10_000);
  for y in 0..10_000 {
    map.insert(Position{x, y}, 0);
  }
  map.len()
}

fn main() {
  let mut result = 0;
  let start = Instant::now();
  for x in 0..10_000 {
    result += everybody_move(x);
  }
  println!("Time elapsed is: {:?} = {result}", start.elapsed());
}
