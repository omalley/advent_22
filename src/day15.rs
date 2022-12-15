use std::cmp::Ordering;
use std::ops::Range;

type InputType = Vec<Sensor>;
type OutputType = usize;

#[derive(Clone,Debug)]
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

pub fn part2(input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day15::{generator, get_unavailable_at_row, part2};


  #[test]
  fn test_part1() {
    let sensors = generator(INPUT);
    assert_eq!(26, get_unavailable_at_row(&sensors, 10));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(93, part2(&generator(INPUT)));
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
