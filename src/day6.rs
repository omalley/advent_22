type InputType = str;
type OutputType = usize;

pub fn generator(input: &str) -> &InputType {
  input
}

fn is_all_unique(w: &[(usize, char)]) -> bool {
  for i in 0..(w.len()-1) {
    for j in i+1..w.len() {
      if w[i].1 == w[j].1 {
        return false
      }
    }
  }
  true
}

fn find_unique(input: &str, size: usize) -> usize {
  let chars: Vec<(usize, char)> = input.chars().enumerate().collect();
  chars.windows(size).find(| &w | is_all_unique(w)).unwrap().first().unwrap().0 + size
}

pub fn part1(input: &InputType) -> OutputType {
  find_unique(input, 4)
}

pub fn part2(input: &InputType) -> OutputType {
  find_unique(input, 14)
}

#[cfg(test)]
mod tests {
  use crate::day6::{part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
  }

  #[test]
  fn test_part2() {
    assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
  }
}
