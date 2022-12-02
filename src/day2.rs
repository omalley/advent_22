#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Move {
  ROCK,
  PAPER,
  SCISSORS,
}

impl Move {
  /// Convert a string to a move
  fn from_str(s: &str) -> Move {
    match s {
      "A" | "X" => Self::ROCK,
      "B" | "Y" => Self::PAPER,
      "C" | "Z" => Self::SCISSORS,
      _ => panic!("can't parse {s}"),
    }
  }

  /// Parse a line into the two moves
  fn parse(s: &str) -> Vec<Self> {
    s.split_whitespace().map(|s| Self::from_str(s)).collect()
  }

  /// The ordinal of the move
  fn value(&self) -> i32 {
    *self as i32
  }

  /// Convert from the ordinal
  fn from_i32(v: i32) -> Self {
    match v {
      0 => Self::ROCK,
      1 => Self::PAPER,
      2 => Self::SCISSORS,
      _ => panic!("bad value {v}"),
    }
  }

  /// Compute the score of a turn.
  fn turn_eval(&self, other: &Self) -> i32 {
    let result_points = match (self.value() - other.value() + 3) % 3 {
      0 => 3,
      1 => 6,
      2 => 0,
      _ => panic!("bad difference {:?} - {:?}", self, other)
    };
    self.value() + 1 + result_points
  }

  /// Find our required move from the result of the turn
  /// and the other player's move.
  /// self: X/Rock => lose, Y/Paper => tie, Z/Scissors => win
  fn find_my_move(&self, other: &Self) -> Self {
    // subtract one from the result so that Y == 0
    Self::from_i32((self.value() - 1 + other.value() + 3) % 3)
  }
}

pub fn generator(input: &str) -> Vec<Vec<Move>> {
  input.lines().map(|l| Move::parse(l)).collect()
}

pub fn part1(input: &[Vec<Move>]) -> i32 {
  input.iter()
    .map(|v| v[1].turn_eval(&v[0]))
    .fold(0, |l, r| l + r)
}

pub fn part2(input: &[Vec<Move>]) -> i32 {
  input.iter()
    .map(|v| v[1].find_my_move(&v[0]).turn_eval(&v[0]))
    .fold(0, |l, r| l + r)
}

#[cfg(test)]
mod tests {
  use crate::day2::{generator, Move, part1, part2};

  const INPUT: &str = "A Y\n\
                       B X\n\
                       C Z";

  #[test]
  fn parsing_test() {
    let moves = generator(INPUT);
    assert_eq!(vec! {vec! {Move::ROCK, Move::PAPER},
                     vec! {Move::PAPER, Move::ROCK},
                     vec! {Move::SCISSORS, Move::SCISSORS}}, moves);
  }

  #[test]
  fn test_part1() {
    assert_eq!(15, part1(&generator(INPUT)))
  }

  #[test]
  fn test_part2() {
    assert_eq!(12, part2(&generator(INPUT)))
  }
}
