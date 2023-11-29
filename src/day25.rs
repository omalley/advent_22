type InputType = Vec<String>;
type OutputType = String;

pub fn generator(input: &str) -> InputType {
  input.lines().map(|s| s.to_string()).collect()
}

const BASE: i64 = 5;

fn char_to_snafu_digit(ch: char) -> i8 {
  match ch {
    '0'..='2' => ch as i8 - '0' as i8,
    '-' => -1,
    '=' => -2,
    _ => panic!("Can't parse '{ch}'"),
  }
}

fn snafu_digit_to_char(digit: i8) -> char {
  match digit {
    0..=2 => (b'0' + digit as u8) as char,
    -1 => '-',
    -2 => '=',
    _ => panic!("Can't handle digit {digit}"),
  }
}

fn snafu_to_i64(s: &str) -> i64 {
  s.chars().map(char_to_snafu_digit)
    .fold(0, |acc, val| acc * BASE + val as i64)
}

fn i64_to_snafu(val: i64) -> String {
  let mut digits: Vec<i8> = Vec::new();
  let mut remaining = val;
  let mut borrow = false;
  while remaining > 0 {
    let mut digit = (remaining % BASE) as i8;
    if borrow {
      digit += 1;
      borrow = false;
    }
    if digit > 2 {
      digit -= 5;
      borrow = true;
    }
    digits.push(digit);
    remaining /= BASE;
  }
  if borrow {
    digits.push(1);
  }
  digits.iter().rev().map(|d| snafu_digit_to_char(*d)).collect::<String>()
}

pub fn part1(input: &InputType) -> OutputType {
  i64_to_snafu(input.iter().map(|l| snafu_to_i64(l)).sum())
}

pub fn part2(_input: &InputType) -> OutputType {
  String::new()
}

#[cfg(test)]
mod tests {
  use crate::day25::{generator, part1};

  #[test]
  fn test_part1() {
    assert_eq!("2=-1=0", part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
  }

  const INPUT: &str =
"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";
}
