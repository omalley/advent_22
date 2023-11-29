/// Define a set of items using a long
#[derive(Debug,Default)]
pub struct Contents {
  items: u64,
}

impl Contents {
  const SIZE: usize = 56;

  fn set(&mut self, posn: usize) {
    self.items |= 1u64 << posn;
  }

  /// Parse from a list o characters
  fn from_chars(chars: &[char]) -> Self {
    let mut result = Self::default();
    for ch in chars {
      result.set(Self::position(*ch));
    }
    result
  }

  /// Map the characters to position in the array
  fn position(ch: char) -> usize {
    match ch {
      'a'..='z' => (ch as usize) - ('a' as usize),
      'A'..='Z' => (ch as usize) - ('A' as usize) + 26,
      _ => panic!("Bad value {ch}")
    }
  }

  /// Map the position back into a character
  fn item(pos: usize) -> char {
    match pos {
      0..=25 => char::from_u32(('a' as u32) + pos as u32).unwrap(),
      26..=51 => char::from_u32(('A' as u32) + pos as u32 - 26).unwrap(),
      _ => panic!("Bad position {pos}")
    }
  }

  /// Create the intersection of a list of sets
  fn intersect(sets: &[Self]) -> Self {
    let mut result = Contents{items: u64::MAX};
    for s in sets {
      result.items &= s.items;
    }
    result
  }

  /// Create the union oof a list of sets
  fn union(sets: &[Self]) -> Self {
    let mut result = Self::default();
    for s in sets {
      result.items |= s.items;
    }
    result
  }

  /// Return the first item in the set
  fn first_item(&self) -> Option<char> {
    let posn = self.items.trailing_zeros() as usize;
    if posn <= Self::SIZE { Some(Self::item(posn)) } else { None }
  }
}

fn priority(ch: char) -> i32 {
  Contents::position(ch) as i32 + 1
}

/// Define Rucksacks as lists of sets
#[derive(Debug)]
pub struct Rucksack {
  parts: Vec<Contents>,
}

impl Rucksack {

  /// Parse a rucksack from a string.
  /// Divides the string in half to get two compartments.
  fn from_str(s: &str) -> Self {
    let chars: Vec<char> = s.chars().collect();
    assert_eq!(chars.len() % 2, 0);
    let parts = chars.chunks(chars.len() / 2)
      .map(Contents::from_chars).collect();
    Rucksack{parts}
  }

  /// Find the common item between the compartments
  fn find_match(&self) -> Option<char> {
    Contents::intersect(&self.parts).first_item()
  }

  /// Get the set of all items in the rucksack.
  fn items(&self) -> Contents {
    Contents::union(&self.parts)
  }
}

/// Find the common item in a group of rucksacks
fn find_match(group: &[Rucksack]) -> Option<char> {
  let contents: Vec<Contents> = group.iter().map(|r| r.items()).collect();
  Contents::intersect(&contents).first_item()
}

pub fn generator(input: &str) -> Vec<Rucksack> {
  input.lines().map(Rucksack::from_str).collect()
}

/// Find the common item in each sack and sum the priorities.
pub fn part1(input: &[Rucksack]) -> i32 {
  input.iter().map(|r| priority(r.find_match().unwrap())).sum()
}

/// Group the sacks into sets of 3, find the common item, and sum the priorities.
pub fn part2(input: &[Rucksack]) -> i32 {
  input.chunks(3).map(|group| priority(find_match(group).unwrap())).sum()
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

  #[test]
  fn test_part1() {
    assert_eq!(157, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(70, part2(&generator(INPUT)));
  }
}
