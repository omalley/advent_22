use std::ops::Range;

type InputType = Vec<Point>;
type OutputType = usize;

type Coordinate = i32;

#[derive(Clone,Debug)]
pub struct Point {
  x: Coordinate,
  y: Coordinate,
  z: Coordinate,
}

impl Point {
  fn parse(line: &str) -> Self {
    let nums: Vec<Coordinate> = line.split(",")
      .map(|w| w.parse::<Coordinate>().unwrap())
      .collect();
    Point{x: nums[0], y: nums[1], z: nums[2]}
  }
}

fn find_range(input: &[Coordinate]) -> Option<Range<Coordinate>> {
  let result = input.iter()
    .fold((Coordinate::MAX, Coordinate::MIN),
          |acc, v| (*v.min(&acc.0), *v.max(&acc.1)));
  if result.0 > result.1 {
    None
  } else {
    Some(result.0..result.1+1)
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines().map(|l| Point::parse(l)).collect()
}

fn size(input: &Range<Coordinate>) -> usize {
  (input.end - input.start) as usize
}

struct Blob {
  ranges: [Range<Coordinate>; 3],
  boxes: Vec<Vec<Vec<bool>>>,
}

impl Blob {
  fn new(input: &[Point]) -> Self {
    let ranges = [
      find_range(&input.iter().map(|p| p.x).collect::<Vec<Coordinate>>()).unwrap(),
      find_range(&input.iter().map(|p| p.y).collect::<Vec<Coordinate>>()).unwrap(),
      find_range(&input.iter().map(|p| p.z).collect::<Vec<Coordinate>>()).unwrap()];
    let boxes =
      vec![vec![vec![false; size(&ranges[2])]; size(&ranges[1])];
           size(&ranges[0])];
    Blob{boxes, ranges}
  }

  fn set(&mut self, point: &Point) {
    self.boxes[(point.x - self.ranges[0].start) as usize]
      [(point.y - self.ranges[1].start) as usize]
      [(point.z - self.ranges[2].start) as usize] = true;
  }

  fn get(&self, point: &Point) -> bool {
    self.boxes[(point.x - self.ranges[0].start) as usize]
      [(point.y - self.ranges[1].start) as usize]
      [(point.z - self.ranges[2].start) as usize]
  }

  fn get_neighbors(&self, point: &Point) -> usize {
    let mut result = 0;
    for delta in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] {
      let other = Point{x: point.x + delta.0, y: point.y + delta.1, z: point.z + delta.2};
      if self.ranges[0].contains(&other.x) && self.ranges[1].contains(&other.y) &&
        self.ranges[2].contains(&other.z) {
        if self.get(&other) {
          result += 1;
        }
      }
    }
    result
  }
}

pub fn part1(input: &InputType) -> OutputType {
  let mut blob = Blob::new(input);
  let mut neighbors: usize = 0;
  for b in input {
    blob.set(b);
    neighbors += blob.get_neighbors(b);
  }
  input.len() * 6 - neighbors * 2
}

pub fn part2(input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day18::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(13, input.len());
    assert_eq!(64, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(58, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
}
