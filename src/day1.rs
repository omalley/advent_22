use std::cmp;

pub fn generator(input: &str) -> Vec<Vec<i32>> {
  let mut result = Vec::new();
  let mut next = Vec::new();
  for line in input.lines() {
    if let Ok(num) = line.parse() {
      next.push(num);
    } else {
      result.push(next.clone());
      next.clear();
    }
  }
  result
}

pub fn part1(input: &[Vec<i32>]) -> i32 {
  input.iter()
    .map(|v| v.iter().fold(0, |a, &b| a + b))
    .reduce(|a, b| cmp::max(a,b)).unwrap()
}

pub fn part2(input: &[Vec<i32>]) -> i32 {
  let mut calories: Vec<i32> = input.iter()
    .map(|v| v.iter().fold(0, |a, &b| a + b)).collect();
  calories.sort_unstable_by(|a, b| b.cmp(a));
  calories.iter().take(3).fold(0, |a, &b| a + b)
}