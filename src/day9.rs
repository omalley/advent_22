use std::collections::HashSet;

type InputType = Vec<Command>;
type OutputType = usize;

#[derive(Clone,Copy,Debug)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

#[derive(Debug)]
pub struct Command {
  dir: Direction,
  count: usize,
}

impl Command {
  fn parse(line: &str) -> Self {
    let (cmd_str, count_str) = line.split_once(' ').unwrap();
    let count = count_str.parse::<usize>().unwrap();
    let dir = match cmd_str {
      "R" => Direction::Right,
      "U" => Direction::Up,
      "L" => Direction::Left,
      "D" => Direction::Down,
      _ => panic!("Invald command {}", cmd_str),
    };
    Command{dir, count}
  }
}

#[derive(Clone,Debug,Default,Eq,Hash,PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn follow(&mut self, leader: &Position) {
    let (x_delta, y_delta) = (leader.x - self.x, leader.y - self.y);
    // Does the tail need to move?
    if !(-1..=1).contains(&x_delta) || !(-1..=1).contains(&y_delta) {
      self.x += i32::signum(x_delta);
      self.y += i32::signum(y_delta);
    }
  }
}

#[derive(Clone,Debug)]
struct Rope {
  head: Position,
  tails: Vec<Position>,
}

impl Rope {
  fn new(num_tails: usize) -> Self {
    Rope{head: Position::default(), tails: vec!{Position::default(); num_tails}}
  }

  /// Move the head and tail in a given direction
  fn go(&mut self, dir: Direction) {
    match dir {
      Direction::Up => self.head.y -= 1,
      Direction::Right => self.head.x += 1,
      Direction::Down => self.head.y += 1,
      Direction::Left => self.head.x -= 1,
    }
    for t in 0..self.tails.len() {
      if t == 0 {
        self.tails[t].follow(&self.head);
      } else {
        let prev= self.tails[t-1].clone();
        self.tails[t].follow(&prev);
      }
    }
  }

  fn get_tail(&self) -> Position {
    self.tails.last().unwrap().clone()
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines()
    .map(Command::parse)
    .collect()
}

fn run_commands(input: &InputType, num_tails: usize) -> OutputType {
  let mut rope = Rope::new(num_tails);
  let mut spots = HashSet::new();
  for cmd in input {
    for _ in 0..cmd.count {
      rope.go(cmd.dir);
      spots.insert(rope.get_tail());
    }
  }
  spots.len()
}

pub fn part1(input: &InputType) -> OutputType {
  run_commands(input, 1)
}

pub fn part2(input: &InputType) -> OutputType {
  run_commands(input, 9)
}

#[cfg(test)]
mod tests {
  use crate::day9::{generator, part1, part2};

  const INPUT: &str = "R 4\n\
                       U 4\n\
                       L 3\n\
                       D 1\n\
                       R 4\n\
                       D 1\n\
                       L 5\n\
                       R 2";

  const INPUT2: &str = "R 5\n\
                        U 8\n\
                        L 8\n\
                        D 3\n\
                        R 17\n\
                        D 10\n\
                        L 25\n\
                        U 20";
  #[test]
  fn test_part1() {
    assert_eq!(13, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(1, part2(&generator(INPUT)));
    assert_eq!(36, part2(&generator(INPUT2)));
  }
}
