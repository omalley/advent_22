use itertools::Itertools;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Spot {
  Wall,
  Floor,
  Void,
}

impl Spot {
  fn parse(c: char) -> Self {
    match c {
      '#' => Spot::Wall,
      '.' => Spot::Floor,
      ' ' => Spot::Void,
      _ => panic!("Can't parse '{c}'"),
    }
  }
}

#[derive(Debug)]
enum Move {
  Left,
  Right,
  Forward(i32),
}

#[derive(Debug)]
struct Map {
  walls: Vec<Vec<Spot>>,
  size: (i32, i32),
}

impl Map {
  fn parse(input: &str) -> Self {
    let walls:Vec<Vec<Spot>> = input.lines()
      .map(|l| l.chars().map(|c| Spot::parse(c)).collect())
      .collect();
    let max_x = walls.iter().map(|r| r.len()).max().unwrap_or(0) as i32;
    let max_y = walls.len() as i32;
    Map{walls, size: (max_x, max_y)}
  }

  fn get(&self, x: i32, y: i32) -> Spot {
    if (0..self.size.0).contains(&x) && (0..self.size.1).contains(&y) {
      let row = &self.walls[y as usize];
      if x < row.len() as i32 {
        return row[x as usize];
      }
    }
    Spot::Void
  }
}

#[derive(Debug)]
pub struct InputType {
  map: Map,
  moves: Vec<Move>,
}

impl InputType {

  fn parse_moves(line: &str) -> Vec<Move> {
    line.trim().chars().group_by(|ch| ('0'..='9').contains(ch))
      .into_iter()
      .map(| (is_digit, mut itr) |
        if is_digit {
          Move::Forward(itr.collect::<String>().parse::<i32>().unwrap())
        } else {
          match itr.next() {
            Some('R') => Move::Right,
            Some('L') => Move::Left,
            x => panic!("Can't parse {x:?}"),
      }}).collect()
  }
}

type OutputType = i32;

pub fn generator(input: &str) -> InputType {
  let (board_text, move_text) = input.split_once("\n\n").unwrap();
  InputType{map: Map::parse(board_text),
    moves: InputType::parse_moves(move_text)}
}

#[derive(Clone,Copy,Debug)]
enum Direction {
  Right,
  Down,
  Left,
  Up,
}

impl Direction {
  fn left(&self) -> Self {
    match self {
      Direction::Up => Direction::Left,
      Direction::Left => Direction::Down,
      Direction::Down => Direction::Right,
      Direction::Right => Direction::Up,
    }
  }

  fn right(&self) -> Self {
    match self {
      Direction::Up => Direction::Right,
      Direction::Left => Direction::Up,
      Direction::Down => Direction::Left,
      Direction::Right => Direction::Down,
    }
  }

  fn delta(&self) -> (i32, i32) {
    match self {
      Direction::Up => (0, -1),
      Direction::Left => (-1, 0),
      Direction::Down => (0, 1),
      Direction::Right => (1, 0),
    }
  }
}

#[derive(Debug)]
struct State {
  x: i32,
  y: i32,
  facing: Direction,
}

impl State {
  fn new(input: &InputType) -> Self {
    let x = input.map.walls[0].iter().enumerate()
      .find(|(_, &s)| s == Spot::Floor).unwrap().0 as i32;
    State{x, y: 0, facing: Direction::Right}
  }

  fn forward(&mut self, distance: i32, map: &Map) {
    let delta = self.facing.delta();
    for _ in 0..distance {
      let mut next = ((self.x + delta.0).rem_euclid(map.size.0),
                  (self.y + delta.1).rem_euclid(map.size.1));
      loop {
        match map.get(next.0, next.1) {
          Spot::Floor => {
            self.x = next.0;
            self.y = next.1;
            break;
          },
          Spot::Wall => return,
          Spot::Void => {
            next = ((next.0 + delta.0).rem_euclid(map.size.0),
                    (next.1 + delta.1).rem_euclid(map.size.1));
          }
        }
      }
    }
  }

  fn execute(&mut self, mv: &Move, map: &Map) {
    //println!("{self:?} do {mv:?}");
    match mv {
      Move::Left => self.facing = self.facing.left(),
      Move::Right => self.facing = self.facing.right(),
      Move::Forward(n) => self.forward(*n, map),
    }
  }

  fn score(&self) -> i32 {
    let result = 1000 * (self.y + 1) + 4 * (self.x + 1) + self.facing as i32;
    result
  }
}

pub fn part1(input: &InputType) -> OutputType {
  let mut state = State::new(input);
  for mv in &input.moves {
    state.execute(mv, &input.map);
  }
  state.score()
}

pub fn part2(_input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day22::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(6032, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(301, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
";
}
