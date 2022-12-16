use std::cmp::Ordering;
use std::ops::Range;

type InputType = Vec<Sensor>;
type OutputType = usize;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn parse(input: &str) -> Self {
    let (x,y) = input.split_once(", ").unwrap();
    // skip over the x= or y=
    Point{x: x[2..].parse().unwrap(), y: y[2..].parse().unwrap()}
  }
}

#[derive(Clone,Debug)]
pub struct Sensor {
  location: Point,
  closest: Point,
}

impl Sensor {
  fn parse(input: &str) -> Self {
    let (left, right)= input.split_once(": closest beacon is at ").unwrap();
    Sensor{location: Point::parse(&left[10..]), closest: Point::parse(&right)}
  }

  fn min_distance(&self) -> i64 {
    i64::abs(self.location.x - self.closest.x) +
        i64::abs(self.location.y - self.closest.y)
  }

  fn invalid_range_on(&self, y: i64) -> Option<Range<i64>> {
    let remaining_dist = self.min_distance() - i64::abs(self.location.y - y);
    if remaining_dist <= 0 {
      return None
    }
    Some(Range{start: self.location.x - remaining_dist,
      end: self.location.x + remaining_dist + 1})
  }

}

pub fn generator(input: &str) -> InputType {
  input.lines().map(|l| Sensor::parse(l)).collect()
}

/// Find and dedup the list of beacons in a given row
fn get_beacons_at_row(input: &InputType, y: i64) -> Vec<i64> {
  let mut result: Vec<i64> = input.iter().filter(|s| s.closest.y == y)
      .map(|s| s.closest.x).collect();
  result.sort_unstable();
  result.dedup();
  result
}

/// Sort and dedup the given ranges.
fn simplify_ranges(ranges: &mut Vec<Range<i64>>) {
  ranges.sort_unstable_by(|r1, r2| match (r1.start.cmp(&r2.start), r1.end.cmp(&r2.end)) {
    (left@(Ordering::Less|Ordering::Greater), _) => left,
    (Ordering::Equal, right) => right,
  });
  let mut i: usize = 0;
  while i < ranges.len() - 1 {
    if ranges[i].end > ranges[i+1].start {
      ranges[i].end = i64::max(ranges[i].end, ranges[i+1].end);
      ranges.remove(i+1);
    } else {
      i += 1;
    }
  }
}

fn ranges_include_location(ranges: &Vec<Range<i64>>, x: i64) -> bool {
  ranges.iter().any(|r| r.start <= x && x < r.end )
}

/// Count the number of locations covered by the ranges
fn count_locations(ranges: &Vec<Range<i64>>) -> usize {
  ranges.iter().map(|r| r.end - r.start).sum::<i64>() as usize
}

/// Count how many spots can't have beacons at them in the given row.
fn get_unavailable_at_row(input: &InputType, y: i64) -> OutputType {
  let mut ranges: Vec<Range<i64>> = input.iter().filter_map(|s| s.invalid_range_on(y)).collect();
  simplify_ranges(&mut ranges);
  // Find the beacon locations that would otherwise count
  let mut beacons = get_beacons_at_row(input, y);
  beacons.retain(|x| ranges_include_location(&ranges, *x));
  count_locations(&ranges) - beacons.len()
}

pub fn part1(input: &InputType) -> OutputType {
  get_unavailable_at_row(input, 2_000_000)
}

/// Points that are rotated by 45 degrees
#[derive(Clone,Debug)]
struct SlantPoint {
  diag_x: i64,
  diag_y: i64,
}

impl SlantPoint {
  fn from(pt: &Point) -> Self {
    SlantPoint{diag_x: pt.x + pt.y, diag_y: pt.y - pt.x}
  }

  fn point(&self) -> Point {
    Point{x: (self.diag_x - self.diag_y)/2, y: (self.diag_x + self.diag_y)/2}
  }

  fn is_valid(&self) -> bool {
    (self.diag_x % 2 == 0) == (self.diag_y % 2 == 0)
  }
}

#[derive(Clone,Debug)]
struct SlantBox {
  left: i64,
  right: i64,
  bottom: i64,
  top: i64,
}

impl SlantBox {
  fn from(sensor: &Sensor) -> Self {
    let dist = sensor.min_distance();
    let mid = SlantPoint::from(&sensor.location);
    SlantBox{bottom: mid.diag_y - dist, top: mid.diag_y + dist + 1,
      left: mid.diag_x - dist, right: mid.diag_x + dist + 1}
  }
}

fn boxify(coord: i64, splits: &[i64]) -> usize {
  match splits.iter().enumerate().find(|(_, &s)| s >= coord) {
    None => splits.len(),
    Some((i, _)) => i,
  }
}

fn find_sensor(input: &InputType, x_range: Range<i64>, y_range: Range<i64>) -> Point {
  let boxes: Vec<SlantBox> = input.iter().map(|s| SlantBox::from(s)).collect();
  let slant_x_bounds = y_range.start + x_range.start .. x_range.end + y_range.end - 1;
  let slant_y_bounds = y_range.start - x_range.end - 1 .. y_range.end - x_range.start;
  let mut x_div: Vec<i64> = boxes.iter()
      .flat_map(|b| vec![b.left, b.right].into_iter()).collect();
  let mut y_div: Vec<i64> = boxes.iter()
      .flat_map(|b| vec![b.bottom, b.top].into_iter()).collect();
  x_div.push(slant_x_bounds.start);
  y_div.push(slant_y_bounds.start);
  x_div.sort_unstable();
  x_div.dedup();
  y_div.sort_unstable();
  y_div.dedup();
  x_div.retain(|x| slant_x_bounds.contains(x));
  y_div.retain(|y| slant_y_bounds.contains(y));
  let mut valid = vec![vec![true; x_div.len()]; y_div.len()];
  for b in &boxes {
    for x in boxify(b.left, &x_div)..boxify(b.right, &x_div) {
      for y in boxify(b.bottom, &y_div)..boxify(b.top, &y_div) {
        valid[y][x] = false;
      }
    }
  }
  for y in 0..valid.len(){
    for x in 0..valid[y].len() {
      if valid[y][x] {
        let slant = SlantPoint{diag_x: x_div[x], diag_y: y_div[y]};
        if slant.is_valid() {
          let pt = slant.point();
          if x_range.contains(&pt.x) && y_range.contains(&pt.y) {
            return pt;
          }
        }
      }
    }
  }
  Point{x:0, y:0}
}

const PART2_LIMIT: i64 = 4_000_000;

pub fn part2(input: &InputType) -> OutputType {
  let pt = find_sensor(input, 0..PART2_LIMIT+1, 0..PART2_LIMIT+1);
  (pt.x * PART2_LIMIT + pt.y) as usize
}

#[cfg(test)]
mod tests {
  use crate::day15::{find_sensor, generator, get_unavailable_at_row, Point};


  #[test]
  fn test_part1() {
    let sensors = generator(INPUT);
    assert_eq!(26, get_unavailable_at_row(&sensors, 10));
  }

  #[test]
  fn test_part2() {
    assert_eq!(Point{x: 14, y:11},
               find_sensor(&generator(INPUT), 0..21, 0..21));
  }

  const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15\n\
                       Sensor at x=9, y=16: closest beacon is at x=10, y=16\n\
                       Sensor at x=13, y=2: closest beacon is at x=15, y=3\n\
                       Sensor at x=12, y=14: closest beacon is at x=10, y=16\n\
                       Sensor at x=10, y=20: closest beacon is at x=10, y=16\n\
                       Sensor at x=14, y=17: closest beacon is at x=10, y=16\n\
                       Sensor at x=8, y=7: closest beacon is at x=2, y=10\n\
                       Sensor at x=2, y=0: closest beacon is at x=2, y=10\n\
                       Sensor at x=0, y=11: closest beacon is at x=2, y=10\n\
                       Sensor at x=20, y=14: closest beacon is at x=25, y=17\n\
                       Sensor at x=17, y=20: closest beacon is at x=21, y=22\n\
                       Sensor at x=16, y=7: closest beacon is at x=15, y=3\n\
                       Sensor at x=14, y=3: closest beacon is at x=15, y=3\n\
                       Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
