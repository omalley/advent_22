use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};
use std::ops::Range;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone,Copy,Debug,EnumIter)]
enum Direction {
  NorthWest,
  North,
  NorthEast,
  West,
  East,
  SouthWest,
  South,
  SouthEast,
}

/// Represent coordinates as i32.
type Coordinate = i32;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct Position {
  x: Coordinate,
  y: Coordinate,
}

impl Position {
  fn advance(&self, toward: Direction) -> Position {
    match toward {
      Direction::NorthWest => Position{x: self.x - 1, y: self.y - 1},
      Direction::North => Position{x: self.x, y: self.y - 1},
      Direction::NorthEast => Position{x: self.x + 1, y: self.y - 1},
      Direction::East => Position{x: self.x + 1, y: self.y},
      Direction::SouthEast => Position{x: self.x + 1, y: self.y + 1},
      Direction::South => Position{x: self.x, y: self.y + 1},
      Direction::SouthWest => Position{x: self.x - 1, y: self.y + 1},
      Direction::West => Position{x: self.x - 1, y: self.y},
    }
  }
}

#[derive(Debug)]
pub struct InputType {
  elves: Vec<Position>,
}

type OutputType = i32;

pub fn generator(input: &str) -> InputType {
  let elves = input.lines().enumerate()
    .flat_map(|(line_num, line) | line.chars().enumerate()
      .filter(|(_, ch)| *ch == '#')
      .map(move |(column, _)| Position{x: column as Coordinate, y: line_num as Coordinate}))
    .collect();
  InputType{elves}
}

#[derive(Debug)]
struct State {
  elves: Vec<Position>,
  first_rule: usize,
  turn: usize,
}

impl State {
  const NUM_RULES: usize = 4;
  fn new(input: &InputType) -> Self {
    State{elves: input.elves.clone(), first_rule: 0, turn: 0}
  }

  fn update_mask(mask: &mut u8, elf: Position, dir: Direction, locations: &HashSet<Position>) {
    if locations.contains(&elf.advance(dir)) {
      *mask |= 1 << (dir as usize);
    }
  }

  fn find_rule(&self, elf: Position, locations: &HashSet<Position>) -> Option<Direction> {
    let mut neighbors: u8 = 0;
    for dir in Direction::iter() {
      Self::update_mask(&mut neighbors, elf, dir, locations);
    }
    if neighbors == 0 {
      return None;
    }
    for rule in 0..Self::NUM_RULES {
      match (rule + self.first_rule) % Self::NUM_RULES {
        0 => if neighbors & 0x7 == 0 { return Some(Direction::North); },
        1 => if neighbors & 0xe0 == 0 { return Some(Direction::South); },
        2 => if neighbors & 0x29 == 0 { return Some(Direction::West); },
        3 => if neighbors & 0x94 == 0 { return Some(Direction::East); },
        x => panic!("Invalid rule number {}", x),
      }
    }
    None
  }

  fn everybody_move(&mut self) -> bool {
    let num_elves = self.elves.len();
    let locations = self.elves.iter().cloned().collect::<HashSet<Position>>();
    let mut result = false;
    // Generate all of the proposals with a count for each location
    let mut proposals: HashMap<Position, usize> = HashMap::with_capacity(num_elves);
    let mut elf_proposal: Vec<Option<Direction>> = Vec::with_capacity(num_elves);
    for elf in &self.elves {
      let rule = self.find_rule(*elf, &locations);
      elf_proposal.push(rule);
      if let Some(dir) = rule {
        let new_loc = elf.advance(dir);
        if let Some(cnt) = proposals.get_mut(&new_loc) {
          *cnt += 1;
        } else {
          proposals.insert(new_loc, 1);
        }
      }
    }
    // If only 1 elf proposed moving there, go ahead and move
    for i in 0..num_elves {
      if let Some(dir) = elf_proposal[i] {
        let new_loc = self.elves[i].advance(dir);
        if *proposals.get(&new_loc).unwrap() == 1 {
          result = true;
          self.elves[i] = new_loc;
        }
      }
    }
    self.first_rule = (self.first_rule + 1) % Self::NUM_RULES;
    self.turn += 1;
    result
  }

  /// Find the x and y ranges of the positions.
  fn find_range(&self) -> (Range<Coordinate>, Range<Coordinate>) {
    if self.elves.is_empty() {
      return (0..0, 0..0);
    }
    let mut x_range = Coordinate::MAX..Coordinate::MIN;
    let mut y_range = Coordinate::MAX..Coordinate::MIN;
    for p in &self.elves {
      x_range.start = x_range.start.min(p.x);
      x_range.end = x_range.end.max(p.x + 1);
      y_range.start = y_range.start.min(p.y);
      y_range.end = y_range.end.max(p.y + 1);
    }
    (x_range, y_range)
  }

  /// Find the total size of the covered area
  fn find_size(&self) -> OutputType {
    let (x_range, y_range) = self.find_range();
    (x_range.end - x_range.start) * (y_range.end - y_range.start)
  }
}

impl Display for State {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let (x_range, y_range) = self.find_range();
    write!(f, "turn: {}, x: {x_range:?}, y: {y_range:?}\n", self.turn)?;
    let locs = self.elves.iter().cloned().collect::<HashSet<Position>>();
    for y in y_range.clone() {
      for x in x_range.clone() {
        if locs.contains(&Position{x,y}) {
          write!(f, "#")?;
        } else {
          write!(f, ".")?;
        }
      }
      write!(f, "\n")?;
    }
    Ok(())
  }
}
const NUM_TURNS: usize = 10;

pub fn part1(input: &InputType) -> OutputType {
  let mut state = State::new(input);
  for _ in 0..NUM_TURNS {
    state.everybody_move();
  }
  state.find_size() - input.elves.len() as OutputType
}

pub fn part2(input: &InputType) -> OutputType {
  let mut state = State::new(input);
  while state.everybody_move() {
    // nothing
  }
  state.turn as OutputType
}

#[cfg(test)]
mod tests {
  use crate::day23::{generator, part1, part2};

  #[test]
  fn test_tiny() {
    let tiny = "##\n\
                      #\n\
                      \n\
                      ##";
    let input = generator(tiny);
    assert_eq!(25, part1(&input));
  }

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(110, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(20, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
}
