use std::cmp::Reverse;
use priority_queue::PriorityQueue;

type InputType = Puzzle;
type OutputType = usize;

#[derive(Clone,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
struct Position {
  x: usize,
  y: usize,
}

#[derive(Debug)]
pub struct Puzzle {
  elevations: Vec<Vec<u8>>,
  width: usize,
  height: usize,
  start: Position,
  end: Position,
}

impl Puzzle {
  fn parse(input: &str) -> Self {
    let mut start = Position::default();
    let mut end = Position::default();
    let mut elevations = Vec::new();
    for line in input.lines() {
      let mut row = Vec::new();
      for ch in line.chars() {
        match ch {
          'a'..='z' => { row.push((ch as u8) - ('a' as u8)); },
          'S' => { start = Position{x: row.len(), y: elevations.len()}; row.push(0u8); },
          'E' => { end = Position{x: row.len(), y: elevations.len()}; row.push(25u8); },
          _ => panic!("Unknown character {ch}"),
        }
      }
      elevations.push(row);
    }
    let height = elevations.len();
    let width = if height == 0 { 0 } else { elevations[0].len() };
    Puzzle{ elevations, width, height, start, end}
  }

  fn next(&self, pos: &Position) -> Vec<Position> {
    let mut result = Vec::new();
    let elevation = self.elevations[pos.y][pos.x] + 1;
    if pos.x > 0 && self.elevations[pos.y][pos.x - 1] <= elevation {
      result.push(Position{x: pos.x - 1, y: pos.y})
    }
    if pos.x < self.width - 1 && self.elevations[pos.y][pos.x + 1] <= elevation{
      result.push(Position{x: pos.x + 1, y: pos.y})
    }
    if pos.y > 0 && self.elevations[pos.y - 1][pos.x] <= elevation{
      result.push(Position{x: pos.x, y: pos.y - 1})
    }
    if pos.y < self.height - 1 && self.elevations[pos.y + 1][pos.x] <= elevation{
      result.push(Position{x: pos.x, y: pos.y + 1})
    }
    result
  }
}

pub fn generator(input: &str) -> InputType {
  Puzzle::parse(input)
}

pub fn part1(input: &InputType) -> OutputType {
  let mut distance = vec!{vec!{usize::MAX; input.width}; input.height};
  distance[input.start.y][input.start.x] = 0;
  let mut queue = PriorityQueue::new();
  queue.push(input.start.clone(), Reverse(0));
  while let Some((current, Reverse(dist))) = queue.pop() {
    if current == input.end {
      return dist
    }
    for next in input.next(&current) {
      if dist + 1 < distance[next.y][next.x] {
        distance[next.y][next.x] = dist + 1;
        queue.push(next, Reverse(dist + 1));
      }
    }
  }
  distance[input.end.y][input.end.x]
}

pub fn part2(input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day12::{generator, part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!(31, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(2713310158, part2(&generator(INPUT)));
  }

  const INPUT: &str = "Sabqponm\n\
                       abcryxxl\n\
                       accszExk\n\
                       acctuvwj\n\
                       abdefghi";
}
