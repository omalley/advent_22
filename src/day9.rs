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
    let (cmd_str, count_str) = line.split_once(" ").unwrap();
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

#[derive(Clone,Debug,Default,Eq,PartialEq)]
struct Position {
  x: i32,
  y: i32,
  tail_x: i32,
  tail_y: i32,
}

impl Position {
  /// Move the head and tail in a given direction
  fn go(&mut self, dir: Direction) {
    match dir {
      Direction::Up => self.y -= 1,
      Direction::Right => self.x += 1,
      Direction::Down => self.y += 1,
      Direction::Left => self.x -= 1,
    }
    let (x_delta, y_delta) = (self.x - self.tail_x, self.y - self.tail_y);
    // Does the tail need to move?
    if x_delta < -1 || x_delta > 1 || y_delta < -1 || y_delta > 1 {
      self.tail_x += i32::signum(x_delta);
      self.tail_y += i32::signum(y_delta);
    }
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines()
    .map(|l| Command::parse(l))
    .collect()
}

pub fn part1(input: &InputType) -> OutputType {
  let mut posn = Position::default();
  let mut spots = HashSet::new();
  for cmd in input {
    for _ in 0..cmd.count {
      posn.go(cmd.dir);
      spots.insert((posn.tail_x, posn.tail_y));
    }
  }
  spots.len()
}

pub fn part2(input: &InputType) -> OutputType {
  0
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

  #[test]
  fn test_part1() {
    assert_eq!(13, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(8, part2(&generator(INPUT)));
  }
}
