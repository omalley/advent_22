use std::collections::HashSet;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone,Copy,Debug,EnumIter,Eq,PartialEq)]
enum Direction {
  Still,
  North,
  West,
  East,
  South,
}

impl Direction {
  fn parse(ch: char) -> Option<Self> {
    match ch {
      '^' => Some(Direction::North),
      '<' => Some(Direction::West),
      '>' => Some(Direction::East),
      'v' => Some(Direction::South),
      '#' => Some(Direction::Still),
      '.' => None,
      _ => panic!("Can't parse '{ch}'"),
    }
  }
}

/// Represent coordinates as i16.
type Coordinate = i16;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Position {
  x: Coordinate,
  y: Coordinate,
}

impl Position {
  fn advance(&self, dir: Direction) -> Self {
    match dir {
      Direction::Still => self.clone(),
      Direction::North => Position{x: self.x, y: self.y - 1},
      Direction::South => Position{x: self.x, y: self.y + 1},
      Direction::West => Position{x: self.x - 1, y: self.y},
      Direction::East => Position{x: self.x + 1, y: self.y},
    }
  }
}

#[derive(Debug)]
pub struct InputType {
  obstacles: Vec<Vec<Option<Direction>>>,
  size: Position,
  start: Position,
  end: Position,
  period: usize,
}

impl InputType {
  ///
  fn get(&self, pos: Position) -> Option<Direction> {
    if (0..self.size.x).contains(&pos.x) && (0..self.size.y).contains(&pos.y) {
      return self.obstacles[pos.y as usize][pos.x as usize];
    }
    Some(Direction::Still)
  }

  /// Wrap positions around within the canyon.
  fn normalize(&self, pos: Position) -> Position {
    Position{x: (pos.x - 1).rem_euclid(self.size.x - 2) + 1,
      y: (pos.y - 1).rem_euclid(self.size.y - 2) + 1}
  }

  /// Is the given location and time safe from an obstacle moving in the given direction.
  fn is_safe_specific(&self, pos: Position, time: usize, dir: Direction) -> bool {
    let delta = time as Coordinate;
    let adjusted_loc = match dir {
      Direction::Still => pos,
      Direction::West => self.normalize(Position{x: pos.x + delta, y: pos.y}),
      Direction::East => self.normalize(Position{x: pos.x - delta, y: pos.y}),
      Direction::North => self.normalize(Position{x: pos.x, y: pos.y + delta}),
      Direction::South => self.normalize(Position{x: pos.x, y: pos.y - delta}),
    };
    let result = self.get(adjusted_loc);
    //println!(" is_safe_specific {pos:?} at {time} for {dir:?} = {adjusted_loc:?} ({result:?})");
    result != Some(dir)
  }

  /// Is the given position safe at the given time?
  fn is_safe(&self, pos: Position, time: usize) -> bool {
    if pos.x <= 0 || pos.x >= self.size.x - 1 || pos.y <= 0 || pos.y >= self.size.y - 1 {
      return self.get(pos).is_none();
    }
    // Where in the cycle are we?
    let cycle = time % self.period;
    for dir in Direction::iter() {
      if !self.is_safe_specific(pos, cycle, dir) {
        return false;
      }
    }
    true
  }
}

type OutputType = usize;

pub fn generator(input: &str) -> InputType {
  let obstacles: Vec<Vec<Option<Direction>>> = input.lines()
    .map(|l| l.chars().map(|c| Direction::parse(c)).collect())
    .collect();
  let start = Position{y: 0,
    x: obstacles[0].iter().enumerate()
      .find(|(_, &o)| o.is_none()).unwrap().0 as Coordinate};
  let rows = obstacles.len();
  let end = Position{y: (rows - 1) as Coordinate,
    x: obstacles[rows - 1].iter().enumerate()
      .find(|(_, o)| o.is_none()).unwrap().0 as Coordinate};
  let width = obstacles.iter().map(|r| r.len()).max().unwrap_or(0);
  let size = Position{x: width as Coordinate, y: rows as Coordinate};
  // What is the time period that all of the obstacles repeat on?
  // We use this so that we don't overflow the i16, even if this goes a long time.
  let period = (width - 2) * (rows - 2);
  InputType{obstacles, start, end, size, period}
}

#[derive(Debug)]
struct State<'a> {
  input: &'a InputType,
  turn: usize,
  locations: HashSet<Position>,
}

impl<'a> State<'a> {
  fn new(input: &'a InputType) -> Self {
    let mut locations = HashSet::new();
    locations.insert(input.start);
    State{input, turn: 0, locations}
  }

  fn step(&mut self) {
    let mut next = HashSet::new();
    for loc in &self.locations {
      for dir in Direction::iter() {
        next.insert(loc.advance(dir));
      }
    }
    next.retain(|loc| self.input.is_safe(*loc, self.turn + 1));
    self.locations = next;
    self.turn += 1;
  }

  fn done(&self, goal: Position) -> bool {
    self.locations.contains(&goal)
  }
}

pub fn part1(input: &InputType) -> OutputType {
  let mut state = State::new(input);
  while !state.done(input.end) {
    state.step();
  }
  state.turn
}

pub fn part2(input: &InputType) -> OutputType {
  let mut state = State::new(input);
  while !state.done(input.end) {
    state.step();
  }
  state.locations.clear();
  state.locations.insert(input.end);
  while !state.done(input.start) {
    state.step();
  }
  state.locations.clear();
  state.locations.insert(input.start);
  while !state.done(input.end) {
    state.step();
  }
  state.turn
}

#[cfg(test)]
mod tests {
  use crate::day24::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(18, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(54, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
}
