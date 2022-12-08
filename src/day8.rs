use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type InputType = Vec<Vec<i8>>;
type OutputType = usize;

#[derive(Clone,Copy,Debug,EnumIter)]
enum Direction {
  North,
  East,
  South,
  West,
}

struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn go(&mut self, dir: Direction) {
    match dir {
      Direction::North => self.y -= 1,
      Direction::East => self.x += 1,
      Direction::South => self.y += 1,
      Direction::West => self.x -= 1,
    }
  }

  fn in_bounds(&self, width: usize, height: usize) -> bool {
    self.x >= 0 && self.x < width as i32 && self.y >= 0 && self.y < height as i32
  }

  fn height(&self, input: &InputType) -> i8 {
    input[self.y as usize][self.x as usize]
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines()
    .map(|l| l.chars().map(|c| (c as i8) - ('0' as i8)).collect())
    .collect()
}

fn is_visible(input: &InputType) -> Vec<Vec<bool>> {
  let height = input.len();
  let width = input[0].len();
  let mut result: Vec<Vec<bool>> = (0..height).map(|_| vec![false; width]).collect();
  // set the 4 corners
  for x in vec![0, width-1] {
    for y in vec![0, height-1] {
      result[y][x] = true;
    }
  }
  for x in 1..width-1 {
    let mut max = -1;
    for y in 0..height-1 {
      if input[y][x] > max {
        max = input[y][x];
        result[y][x] = true;
      }
    }
    max = -1;
    for y in (1..height).rev() {
      if input[y][x] > max {
        max = input[y][x];
        result[y][x] = true;
      }
    }
  }
  for y in 1..height-1 {
    let mut max = -1;
    for x in 0..width-1 {
      if input[y][x] > max {
        max = input[y][x];
        result[y][x] = true;
      }
    }
    max = -1;
    for x in (1..width).rev() {
      if input[y][x] > max {
        max = input[y][x];
        result[y][x] = true;
      }
    }
  }
  result
}

pub fn part1(input: &InputType) -> OutputType {
  is_visible(input).iter()
    .map(|row| row.iter().map(|l| if *l {1} else {0}).sum::<usize>())
    .sum()
}

fn scenary(input: &InputType, x: usize, y: usize, width: usize, height: usize) -> usize {
  let mut result = 1;
  let our_height = input[y][x];
  for dir in Direction::iter() {
    let mut posn = Position{x: x as i32, y: y as i32};
    let mut trees: usize = 0;
    loop {
      posn.go(dir);
      if !posn.in_bounds(width, height) {
        break;
      }
      trees += 1;
      if posn.height(input) >= our_height {
        break;
      }
    }
    result *= trees;
  }
  result
}

pub fn part2(input: &InputType) -> OutputType {
  let height = input.len();
  let width = input[0].len();
  (1..height-1).map(|y| (1..width-1)
    .map(|x| scenary(input, x, y, width, height)).max().unwrap_or(0))
    .max().unwrap_or(0)
}

#[cfg(test)]
mod tests {
  use crate::day8::{generator, part1, part2};

  const INPUT: &str = "30373\n\
                       25512\n\
                       65332\n\
                       33549\n\
                       35390";

  #[test]
  fn test_generatro() {
    assert_eq!(vec![vec![3,0,3,7,3],
                    vec![2,5,5,1,2],
                    vec![6,5,3,3,2],
                    vec![3,3,5,4,9],
                    vec![3,5,3,9,0]], generator(INPUT));
  }

  #[test]
  fn test_part1() {
    assert_eq!(21, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(8, part2(&generator(INPUT)));
  }
}
