type InputType = Vec<char>;
type OutputType = usize;

pub fn generator(input: &str) -> InputType {
  input.chars().collect()
}

// Find whether all of the chars are unique.
fn is_all_unique(w: &[char]) -> bool {
  for i in 0..(w.len()-1) {
    if w[i+1..].contains(&w[i]) {
      return false
    }
  }
  true
}

fn find_unique(input: &Vec<char>, size: usize) -> usize {
  for posn in 0..(input.len()-size) {
    if is_all_unique(&input[posn..posn+size]) {
      return posn + size
    }
  }
  usize::MAX
}

pub fn part1(input: &InputType) -> OutputType {
  find_unique(input, 4)
}

pub fn part2(input: &InputType) -> OutputType {
  find_unique(input, 14)
}

#[cfg(test)]
mod tests {
  use crate::day6::{generator, part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!(7, part1(&generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    assert_eq!(5, part1(&generator("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(6, part1(&generator("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(10, part1(&generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(11, part1(&generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
  }

  #[test]
  fn test_part2() {
    assert_eq!(19, part2(&generator("mjqjpqmgbljsphdztnvjfqwrcgsmlb")));
    assert_eq!(23, part2(&generator("bvwbjplbgvbhsrlpgdmjqwftvncz")));
    assert_eq!(23, part2(&generator("nppdvjthqldpwncqszvftbrmjlhg")));
    assert_eq!(29, part2(&generator("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")));
    assert_eq!(26, part2(&generator("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")));
  }
}
