use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

type InputType = Vec<(List,List)>;
type OutputType = usize;

#[derive(Clone,Debug)]
pub enum List {
  Int(i64),
  List(Vec<RefCell<List>>),
}

impl List {
  fn parse_num(chars: &[char]) -> (List, usize) {
    let num: String = chars.iter().take_while(|&&c| c >= '0' && c <= '9').collect();
    (List::Int(num.parse::<i64>().expect("number")), num.len())
  }

  /// Parse a list and return the list and how many characters were used
  fn parse_list(chars: &[char]) -> (List, usize) {
    let mut posn: usize = 0;
    if chars[posn] != '[' {
      panic!("expected '[' at {:?}", chars);
    }
    posn += 1;
    let mut list: Vec<RefCell<List>> = Vec::new();
    while chars[posn] != ']' {
      match chars[posn] {
        '0'..='9' => {
          let (val, next) = Self::parse_num(&chars[posn..]);
          list.push(RefCell::new(val));
          posn += next;
        },
        ',' => {
          posn += 1;
        },
        '[' => {
          let (val, next) = Self::parse_list(&chars[posn..]);
          list.push(RefCell::new(val));
          posn += next;
        },
        _ => panic!("Can't handle {:?}", &chars[posn..]),
      }
    }
    posn += 1;
    (List::List(list), posn)
  }

  fn parse(line: &str) -> Self {
    let chars: Vec<char> = line.trim().chars().collect();
    let (val, next) = match chars[0] {
      '0'..='9' => Self::parse_num(&chars),
      '[' => Self::parse_list(&chars),
      _ => panic!("Can't parse {}", line),
    };
    if next != chars.len() {
      panic!("extra stuff in {} at {}", line, next);
    }
    val
  }

  fn make_list(val: i64) -> Vec<RefCell<List>> {
    vec![RefCell::new(Self::Int(val))]
  }

  fn list_cmp(left: &[RefCell<List>], right: &[RefCell<List>]) -> Ordering {
    if left.len() == 0 {
      if right.len() == 0 {
        return Ordering::Equal
      }
      return Ordering::Less
    } else if right.len() == 0 {
      return Ordering::Greater
    }
    let result = left[0].borrow().cmp(&right[0].borrow());
    if result != Ordering::Equal {
      return result
    }
    Self::list_cmp(&left[1..], &right[1..])
  }

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
        write!(f, "[{}]", v.iter().map(|x| format!("{}", x.borrow()))
          .collect::<Vec<String>>().join(","))
      },
    }
  }
}

pub fn generator(input: &str) -> InputType {
  input.split("\n\n")
    .map(|pair| {
      let (first, second) = pair.split_once("\n").unwrap();
      (List::parse(first), List::parse(second))
    }).collect()
}

pub fn part1(input: &InputType) -> OutputType {
  input.iter().enumerate()
    .filter(|(_, (l, r))| l.cmp(r) == Ordering::Less)
    .map(|(idx, _)| idx + 1)
    .sum()
}

const DIVIDER_1: &str = "[[2]]";
const DIVIDER_2: &str = "[[6]]";

pub fn part2(input: &InputType) -> OutputType {
  let mut list: Vec<List> = Vec::new();
  for (l, r) in input {
    list.push(l.clone());
    list.push(r.clone());
  }
  list.push(List::parse(DIVIDER_1));
  list.push(List::parse(DIVIDER_2));
  list.sort_by(|l, r| l.cmp(r));
  list.iter().enumerate().filter(|(_, l)| {
    let str = format!("{}", l);
    str == DIVIDER_1 || str == DIVIDER_2
  }).map(|(i, _)| i + 1).product()
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
