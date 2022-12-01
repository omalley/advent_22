use itertools::Itertools;
use std::cmp;

pub fn generator(input: &str) -> Vec<Vec<i32>> {
  input.lines()
    .map(|line| line.parse().ok())
    .group_by(|x| x.is_some()).into_iter()
    .filter(|(is_valid, _)| *is_valid)
    .map(|(_, group)| group.map(|i| i.unwrap()).collect())
    .collect()
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