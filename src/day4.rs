#[derive(Debug, Eq, PartialEq)]
pub struct Range {
  lower: i32,
  upper: i32,
}

impl Range {
  fn contains(&self, other: &Self) -> bool {
    if self.lower == other.lower || self.upper == other.upper {
      true
    } else if self.lower < other.lower {
      self.upper > other.upper
    } else {
      self.upper < other.upper
    }
  }

  fn overlaps(&self, other: &Self) -> bool {
    let left = i32::max(self.lower, other.lower);
    let right = i32::min(self.upper, other.upper);
    left <= right
  }
}

type InputType = Vec<Vec<Range>>;

pub fn generator(input: &str) -> InputType {
  input.lines()
    .map(|l| l.split(',')
      .filter_map(|r| r.split_once('-'))
      .map(|(l,u)|
        Range{lower: l.parse::<i32>().expect("not integer"),
              upper: u.parse::<i32>().expect("not integer")})
      .collect())
    .collect()
}

/// Sum each group and find the maximum
pub fn part1(input: &InputType) -> i32 {
  input.iter().filter(|pair| pair[0].contains(&pair[1])).count() as i32
}

pub fn part2(input: &InputType) -> i32 {
  input.iter().filter(|pair| pair[0].overlaps(&pair[1])).count() as i32
}

#[cfg(test)]
mod tests {
  use crate::day4::{Range, generator, part1, part2};

  const INPUT: &str = "2-4,6-8\n\
                       2-3,4-5\n\
                       5-7,7-9\n\
                       2-8,3-7\n\
                       6-6,4-6\n\
                       2-6,4-8";

  #[test]
  fn parsing_test() {
    let result= generator("1-2,3-4");
    assert_eq!(vec!{vec!{Range{lower: 1, upper: 2}, Range{lower: 3, upper: 4}}}, result);
  }

  #[test]
  fn test_part1() {
    assert_eq!(2, part1(&generator(INPUT)));
    assert_eq!(0, part1(&generator("10-20,12-22")));
    assert_eq!(1, part1(&generator("10-20,10-10")));
    assert_eq!(1, part1(&generator("10-20,20-20")));
    assert_eq!(1, part1(&generator("10-10,10-20")));
    assert_eq!(1, part1(&generator("20-20,10-20")));
  }

  #[test]
  fn test_part2() {
    assert_eq!(4, part2(&generator(INPUT)));
  }
}
