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
    let nums: Vec<Coordinate> = line.split(',')
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
  input.lines().map(Point::parse).collect()
}

fn size(input: &Range<Coordinate>) -> usize {
  (input.end - input.start) as usize
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Kind {
  Air,
  Rock,
  Outside,
}

#[derive(Debug)]
struct Blob {
  ranges: [Range<Coordinate>; 3],
  boxes: Vec<Vec<Vec<Kind>>>,
}

impl Blob {
  fn new(input: &[Point]) -> Self {
    let ranges = [
      find_range(&input.iter().map(|p| p.x).collect::<Vec<Coordinate>>()).unwrap(),
      find_range(&input.iter().map(|p| p.y).collect::<Vec<Coordinate>>()).unwrap(),
      find_range(&input.iter().map(|p| p.z).collect::<Vec<Coordinate>>()).unwrap()];
    let boxes =
      vec![vec![vec![Kind::Air; size(&ranges[2])]; size(&ranges[1])];
           size(&ranges[0])];
    Blob{boxes, ranges}
  }

  fn set(&mut self, point: &Point, kind: Kind) {
    self.boxes[(point.x - self.ranges[0].start) as usize]
      [(point.y - self.ranges[1].start) as usize]
      [(point.z - self.ranges[2].start) as usize] = kind;
  }

  fn get(&self, point: &Point) -> Kind {
    self.boxes[(point.x - self.ranges[0].start) as usize]
      [(point.y - self.ranges[1].start) as usize]
      [(point.z - self.ranges[2].start) as usize]
  }

  fn get_neighbors(&self, point: &Point) -> usize {
    let mut result = 0;
    for delta in [(1, 0, 0), (-1, 0, 0), (0, 1, 0),
        (0, -1, 0), (0, 0, 1), (0, 0, -1)] {
      let other = Point{x: point.x + delta.0, y: point.y + delta.1, z: point.z + delta.2};
      if self.ranges[0].contains(&other.x) &&
         self.ranges[1].contains(&other.y) &&
         self.ranges[2].contains(&other.z) &&
         self.get(&other) == Kind::Rock {
        result += 1;
      }
    }
    result
  }

  fn get_slice(&self, x_range: &Range<Coordinate>, y_range: &Range<Coordinate>,
               z_range: &Range<Coordinate>) -> Vec<Point> {
    let mut result = Vec::new();
    for x in x_range.clone() {
      for y in y_range.clone() {
        for z in z_range.clone() {
          result.push(Point{x,y,z});
        }
      }
    }
    result
  }

  /// Get the points that make the bounding box of the blob. This will be the points along
  /// all sixes faces.
  fn get_box_edges(&self) -> Vec<Point> {
    let mut result = Vec::new();
    result.extend(self.get_slice(&(self.ranges[0].start..self.ranges[0].start+1),
    &self.ranges[1], &self.ranges[2]));
    result.extend(self.get_slice(&(self.ranges[0].end-1..self.ranges[0].end),
                                 &self.ranges[1], &self.ranges[2]));
    result.extend(self.get_slice(&self.ranges[0],
                                 &(self.ranges[1].start..self.ranges[1].start+1),
                                 &self.ranges[2]));
    result.extend(self.get_slice(&self.ranges[0],
                                 &(self.ranges[1].end-1..self.ranges[1].end),
                                 &self.ranges[2]));
    result.extend(self.get_slice(&self.ranges[0], &self.ranges[1],
                                 &(self.ranges[2].start..self.ranges[2].start+1)));
    result.extend(self.get_slice(&self.ranges[0], &self.ranges[1],
                                 &(self.ranges[2].end-1..self.ranges[2].end)));
    result
  }
}

pub fn part1(input: &InputType) -> OutputType {
  let mut blob = Blob::new(input);
  let mut neighbors: usize = 0;
  for b in input {
    blob.set(b, Kind::Rock);
    neighbors += blob.get_neighbors(b);
  }
  input.len() * 6 - neighbors * 2
}

pub fn part2(input: &InputType) -> OutputType {
  let mut blob = Blob::new(input);
  for b in input {
    blob.set(b, Kind::Rock);
  }
  let mut exterior_faces: usize = 0;
  let mut pending: Vec<Point> = Vec::new();
  for pt in &blob.get_box_edges() {
    match blob.get(pt) {
      Kind::Rock => { exterior_faces += 1; },
      Kind::Air => { pending.push(pt.clone()); }
      Kind::Outside => { }
    }
  }
  while let Some(pt) = pending.pop() {
    // If we've already looked here, just continue.
    if blob.get(&pt) != Kind::Air {
      continue;
    }
    blob.set(&pt, Kind::Outside);
    for delta in [(1, 0, 0), (-1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, 1), (0, 0, -1)] {
      let other = Point { x: pt.x + delta.0, y: pt.y + delta.1, z: pt.z + delta.2 };
      if blob.ranges[0].contains(&other.x) && blob.ranges[1].contains(&other.y) &&
          blob.ranges[2].contains(&other.z) {
        match blob.get(&other) {
          Kind::Rock => { exterior_faces += 1; },
          Kind::Outside => {},
          Kind::Air => { pending.push(other); }
        }
      }
    }
  }
  exterior_faces
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
