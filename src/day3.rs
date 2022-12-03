/// Define a set of items using an array of bool
#[derive(Debug)]
pub struct Contents {
  items: [bool; Contents::SIZE],
}

impl Contents {
  const SIZE: usize = 56;

  /// Parse from a list o characters
  fn from_chars(chars: &[char]) -> Self {
    let mut items = [false; Self::SIZE];
    for ch in chars {
      items[Self::position(*ch)] = true;
    }
    Contents{items}
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
    let mut items = [true; Self::SIZE];
    for s in sets {
      for i in 0..Self::SIZE {
        if !s.items[i] {
          items[i] = false;
        }
      }
    }
    Contents{items}
  }

  /// Create the union oof a list of sets
  fn union(sets: &[Self]) -> Self {
    let mut items = [false; Self::SIZE];
    for s in sets {
      for i in 0..Self::SIZE {
        if s.items[i] {
          items[i] = true;
        }
      }
    }
    Contents{items}
  }

  /// Return the first item in the set
  fn first_item(&self) -> Option<char> {
    for i in 0..Self::SIZE {
      if self.items[i] {
        return Some(Self::item(i))
      }
    }
    None
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
      .map(|ch| Contents::from_chars(ch)).collect();
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
  input.lines().map(|l| Rucksack::from_str(l)).collect()
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
