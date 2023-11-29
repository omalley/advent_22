use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::iter::Peekable;
use std::rc::Rc;
use std::str::Chars;

type InputType = Vec<Rc<List>>;
type OutputType = usize;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum List {
  Int(i64),
  List(Vec<List>),
}

impl List {
  fn parse_digit(chars: &mut Peekable<Chars>) -> i64 {
    (chars.next().unwrap() as i64) - ('0' as i64)
  }

  fn parse_num(chars: &mut Peekable<Chars>) -> List {
    let mut num: i64 = 0;
    loop {
      match chars.peek() {
        Some('0'..='9') => { num = num * 10 + Self::parse_digit(chars); }
        _ => return Self::Int(num),
      }
    }
  }

  /// Parse a list and return the list and how many characters were used
  fn parse_list(chars: &mut Peekable<Chars>) -> List {
    // Skip over the open '['
    chars.next();
    let mut list: Vec<List> = Vec::new();
    loop {
      match chars.peek() {
        Some(']') => { chars.next(); return Self::List(list); },
        Some(',') => { chars.next(); },
        Some(_) => { list.push(Self::parse_from_peekable(chars)); },
        None => panic!("End of stream"),
      }
    }
   }

  fn parse_from_peekable(chars: &mut Peekable<Chars>) -> Self {
    match chars.peek() {
      Some('0'..='9') => Self::parse_num(chars),
      Some('[') => Self::parse_list(chars),
      _ => panic!("Unknown char '{:?}'", chars.peek()),
    }
  }

  fn parse(line: &str) -> Rc<Self> {
    let mut chars = line.chars().peekable();
    Rc::new(Self::parse_from_peekable(&mut chars))
  }

  fn make_list(val: i64) -> Vec<List> {
    vec![List::Int(val)]
  }

  fn list_cmp(left: &[List], right: &[List]) -> Ordering {
    match (left.len(), right.len()) {
      (0, 0) => return Ordering::Equal,
      (0, _) => return Ordering::Less,
      (_, 0) => return Ordering::Greater,
      (_, _) => (),
    }
    match left[0].cmp(&right[0]) {
      Ordering::Equal => Self::list_cmp(&left[1..], &right[1..]),
      result => result,
    }
  }

}

impl PartialOrd<Self> for List {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(List::cmp(self, other))
  }
}

impl Ord for List {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (Self::Int(l), Self::Int(r)) => l.cmp(r),
      (Self::List(l), Self::List(r)) => Self::list_cmp(l, r),
      (Self::Int(l), Self::List(r)) =>
        Self::list_cmp(&Self::make_list(*l), r),
      (Self::List(l), Self::Int(r)) =>
        Self::list_cmp(l, &Self::make_list(*r)),
    }
  }
}

impl Display for List {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Int(i) => write!(f, "{}", i),
      Self::List(v) => {
        write!(f, "[{}]", v.iter().map(|x| format!("{}", x))
          .collect::<Vec<String>>().join(","))
      },
    }
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines()
      .filter(|&l| !l.is_empty())
      .map(List::parse)
      .collect()
}

pub fn part1(input: &InputType) -> OutputType {
  input.chunks(2)
      .enumerate()
    .filter(|(_, pairs)| pairs[0].cmp(&pairs[1]) == Ordering::Less)
    .map(|(idx, _)| idx + 1)
    .sum()
}

pub fn part2(input: &InputType) -> OutputType {
  let mut list = input.clone();
  let dividers = vec![List::parse("[[2]]"), List::parse("[[6]]")];
  list.extend(dividers.iter().cloned());
  list.sort_unstable();
  list.iter().enumerate()
      .filter(|(_, l)| dividers.iter().any(|d| Rc::ptr_eq(l,d)))
      .map(|(i, _)| i + 1).product()
}

#[cfg(test)]
mod tests {
  use std::cmp::Ordering;
  use crate::day13::{generator, List, part1, part2};

  #[test]
  fn test_cmp() {
    let l = List::parse("12");
    let r = List::parse("20");
    assert_eq!(Ordering::Less, l.cmp(&r));
    let l = List::parse("[12]");
    let r = List::parse("[]");
    assert_eq!(Ordering::Greater, l.cmp(&r));
    let l = List::parse("[]");
    let r = List::parse("[12]");
    assert_eq!(Ordering::Less, l.cmp(&r));
    let l = List::parse("2");
    let r = List::parse("[5]");
    assert_eq!(Ordering::Less, l.cmp(&r));
    let l = List::parse("2");
    let r = List::parse("[2]");
    assert_eq!(Ordering::Equal, l.cmp(&r));
    let l = List::parse("[2]");
    let r = List::parse("5");
    assert_eq!(Ordering::Less, l.cmp(&r));
    let l = List::parse("[1,1,3,1,1]");
    let r = List::parse("[1,1,5,1,1]");
    assert_eq!(Ordering::Less, l.cmp(&r));
    let l = List::parse("[[1],[2,3,4]]");
    let r = List::parse("[[1],4]");
    assert_eq!(Ordering::Less, l.cmp(&r));
    let l = List::parse("[1,[2,[3,[4,[5,6,7]]]],8,9]");
    let r = List::parse("[1,[2,[3,[4,[5,6,0]]]],8,9]");
    assert_eq!(Ordering::Greater, l.cmp(&r));
  }

  #[test]
  fn test_part1() {
    assert_eq!(13, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(140, part2(&generator(INPUT)));
  }

  const INPUT: &str = "[1,1,3,1,1]\n\
                       [1,1,5,1,1]\n\
                       \n\
                       [[1],[2,3,4]]\n\
                       [[1],4]\n\
                       \n\
                       [9]\n\
                       [[8,7,6]]\n\
                       \n\
                       [[4,4],4,4]\n\
                       [[4,4],4,4,4]\n\
                       \n\
                       [7,7,7,7]\n\
                       [7,7,7]\n\
                       \n\
                       []\n\
                       [3]\n\
                       \n\
                       [[[]]]\n\
                       [[]]\n\
                       \n\
                       [1,[2,[3,[4,[5,6,7]]]],8,9]\n\
                       [1,[2,[3,[4,[5,6,0]]]],8,9]";
}
