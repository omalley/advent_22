use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

type InputType = Vec<(Rc<List>,Rc<List>)>;
type OutputType = usize;

#[derive(Clone,Debug)]
pub enum List {
  Int(i64),
  List(Vec<Rc<List>>),
}

impl List {
  fn parse_num(chars: &[char]) -> (Rc<List>, usize) {
    let num: String = chars.iter().take_while(|&&c| c >= '0' && c <= '9').collect();
    (Rc::new(List::Int(num.parse::<i64>().expect("number"))), num.len())
  }

  /// Parse a list and return the list and how many characters were used
  fn parse_list(chars: &[char]) -> (Rc<List>, usize) {
    let mut posn: usize = 0;
    if chars[posn] != '[' {
      panic!("expected '[' at {:?}", chars);
    }
    posn += 1;
    let mut list: Vec<Rc<List>> = Vec::new();
    while chars[posn] != ']' {
      match chars[posn] {
        '0'..='9' => {
          let (val, next) = Self::parse_num(&chars[posn..]);
          list.push(val);
          posn += next;
        },
        ',' => {
          posn += 1;
        },
        '[' => {
          let (val, next) = Self::parse_list(&chars[posn..]);
          list.push(val);
          posn += next;
        },
        _ => panic!("Can't handle {:?}", &chars[posn..]),
      }
    }
    posn += 1;
    (Rc::new(List::List(list)), posn)
  }

  fn parse(line: &str) -> Rc<Self> {
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

  fn make_list(val: i64) -> Vec<Rc<List>> {
    vec![Rc::new(List::Int(val))]
  }

  fn list_cmp(left: &[Rc<List>], right: &[Rc<List>]) -> Ordering {
    if left.len() == 0 {
      if right.len() == 0 {
        return Ordering::Equal
      }
      return Ordering::Less
    } else if right.len() == 0 {
      return Ordering::Greater
    }
    let result = left[0].cmp(&right[0]);
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
        write!(f, "[{}]", v.iter().map(|x| format!("{}", x))
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

pub fn part2(input: &InputType) -> OutputType {
  let mut list: Vec<Rc<List>> = Vec::new();
  for (l, r) in input {
    list.push(l.clone());
    list.push(r.clone());
  }
  let dividers = vec![List::parse("[[2]]"), List::parse("[[6]]")];
  list.extend(dividers.iter().cloned());
  list.sort_by(|l, r| l.cmp(r));
  list.iter().enumerate()
      .filter(|(_, l)| dividers.iter().any(|x| Rc::ptr_eq(x, l)))
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
