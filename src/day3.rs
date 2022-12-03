use std::collections::HashSet;

#[derive(Debug)]
pub struct Compartment {
  items: HashSet<char>,
}

impl Compartment {
  fn from_chars(chars: &[char]) -> Self {
    let mut items = HashSet::new();
    for ch in chars {
      items.insert(*ch);
    }
    Compartment{items}
  }
}

#[derive(Debug)]
pub struct Rucksack {
  parts: Vec<Compartment>,
}

fn priority(ch: char) -> i32 {
  match ch {
    'a'..='z' => (ch as i32) - ('a' as i32) + 1,
    'A'..='Z' => (ch as i32) - ('A' as i32) + 27,
    _ => panic!("Bad value {ch}")
  }
}

impl Rucksack {
  fn from_str(s: &str) -> Self {
    let chars: Vec<char> = s.chars().collect();
    assert_eq!(chars.len() % 2, 0);
    let parts = chars.chunks(chars.len() / 2)
      .map(|ch| Compartment::from_chars(ch)).collect();
    Rucksack{parts}
  }

  fn find_match(&self) -> i32 {
    for ch in self.parts[0].items.intersection(&self.parts[1].items) {
      return priority(*ch);
    }
    0
  }

  fn contains(&self, ch: char) -> bool {
    for comp in &self.parts {
      if comp.items.contains(&ch) {
        return true
      }
    }
    false
  }

  fn items(&self) -> Vec<char> {
    self.parts.iter()
      .flat_map(|c| c.items.iter().map(|c| *c).collect::<Vec<char>>())
      .collect()
  }
}

fn find_match(group: &[Rucksack]) -> i32 {
  assert_ne!(group.len(), 0);
  'item: for ch in &group[0].items() {
    for other in &group[1..] {
      if !other.contains(*ch) {
        continue 'item
      }
    }
    return priority(*ch)
  }
  0
}

pub fn generator(input: &str) -> Vec<Rucksack> {
  input.lines().map(|l| Rucksack::from_str(l)).collect()
}

pub fn part1(input: &[Rucksack]) -> i32 {
  input.iter().map(|r| r.find_match()).sum()
}

pub fn part2(input: &[Rucksack]) -> i32 {
  input.chunks(3).map(|group| find_match(group)).sum()
}

#[cfg(test)]
mod tests {
  use crate::day3::{generator, part1, part2};

  const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                       jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                       PmmdzqPrVvPwwTWBwg\n\
                       wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                       ttgJtRGJQctTZtZT\n\
                       CrZsJsPPZsGzwwsLwLmpwMDw";

  fn parsing_test() {
    for x in generator(INPUT) {
      println!("{:?}", x);
    }
  }

  #[test]
  fn test_part1() {
    assert_eq!(157, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(70, part2(&generator(INPUT)));
  }
}
