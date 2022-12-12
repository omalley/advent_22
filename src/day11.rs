use std::collections::VecDeque;

type InputType = Vec<Monkey>;
type OutputType = usize;
type ItemType = u64;
type LiteralType = u64;

#[derive(Clone,Debug)]
enum Operation {
  Add(ItemType),
  Multiply(ItemType),
  Square,
}

impl Operation {
  fn parse(input: &str) -> Result<Self, String> {
    match input.split_whitespace().collect::<Vec<&str>>().as_slice()[3..] {
      ["old", "*", "old"] => Ok(Self::Square),
      ["old", "+", lit] => Ok(Self::Add(parse_lit(lit)?)),
      ["old", "*", lit] => Ok(Self::Multiply(parse_lit(lit)?)),
      _ => Err(format!("Can't parse '{}'", input)),
    }
  }

  fn perform(&self, val: &mut ItemType) {
    match self {
      Self::Add(lit) => { *val += lit },
      Self::Multiply(lit) => { *val *= lit },
      Self::Square => { *val *= *val },
    }
  }
}

fn parse_lit(input: &str) -> Result<ItemType, String> {
  input.parse::<ItemType>().or_else(|e| Err(format!("{}", e)))
}

#[derive(Clone,Debug)]
enum Test {
  Divisble(ItemType),
}

impl Test {
  fn check(&self, val: ItemType) -> bool {
    match self {
      Self::Divisble(lit) => val % lit == 0
    }
  }
}

impl Test {
  fn parse(input: &str) -> Result<Self, String> {
    match input.split_whitespace().collect::<Vec<&str>>().as_slice()[1..] {
      ["divisible", "by", lit] => Ok(Self::Divisble(parse_lit(lit)?)),
      _ => Err(format!("Can't parse '{}'", input)),
    }
  }

  fn divisor(&self) -> LiteralType {
    match self {
      Self::Divisble(lit) => *lit,
    }
  }
}

#[derive(Clone,Copy,Debug)]
pub struct FlyingObject {
  item: ItemType,
  target: usize,
}

#[derive(Clone,Debug)]
pub struct Monkey {
  items: VecDeque<ItemType>,
  operation: Operation,
  test: Test,
  next: (usize, usize),
  inspected: usize,
}

impl Monkey {
  fn parse_items(input: &str) -> Result<VecDeque<ItemType>, String> {
    input.split(", ").map(|item| parse_lit(item)).collect()
  }

  fn parse_next(input: &str) -> Result<usize, String> {
    let word = input.split_whitespace().skip(5).next().ok_or("bad next")?;
    word.parse().or_else(|_| Err(format!("bad next int '{}'", input)))
  }

  fn parse(input: &str) -> Result<Self, String> {
    let lines: Vec<&str> = input.lines().collect();
    let items = Self::parse_items(lines[1].split_once(": ")
      .ok_or("bad items")?.1)?;
    let operation = Operation::parse(lines[2])?;
    let test = Test::parse(lines[3])?;
    let next = (Self::parse_next(lines[4])?, Self::parse_next(lines[5])?);
    Ok(Monkey{items, operation, test, next, inspected: 0})
  }

  fn next_throw(&mut self, clip: impl Fn(ItemType) -> ItemType) -> Option<FlyingObject> {
    let mut item = self.items.pop_front()?;
    self.inspected += 1;
    self.operation.perform(&mut item);
    item = clip(item);
    let target = if self.test.check(item) { self.next.0 } else { self.next.1 };
    Some(FlyingObject{item, target})
  }

  fn catch_object(&mut self, item: ItemType) {
    self.items.push_back(item);
  }
}

pub fn generator(input: &str) -> InputType {
  input.split("\n\n")
    .map(|monkey| Monkey::parse(monkey).unwrap())
    .collect::<Vec<Monkey>>()
}

fn do_round(monkies: &mut Vec<Monkey>, clip: impl Fn(ItemType) -> ItemType) {
  for m in 0..monkies.len() {
    while let Some(flying) = monkies[m].next_throw(&clip) {
      monkies[flying.target].catch_object(flying.item);
    }
  }
}

fn compute_top_two(monkies: &Vec<Monkey>) -> OutputType {
  let mut counts: Vec<usize> = monkies.iter().map(|m| m.inspected).collect();
  counts.sort_by(|l,r| r.cmp(l));
  counts.iter().take(2).product()
}

fn find_multiple(monkies: &Vec<Monkey>, extra_factors: &Vec<ItemType>) -> ItemType {
  let mut factors: Vec<ItemType> = monkies.iter().map(|m| m.test.divisor()).collect();
  factors.extend(extra_factors.iter());
  factors.iter().fold(1, |acc, v| if acc % v == 0 { acc } else { acc * v })
}

pub fn part1(input: &InputType) -> OutputType {
  let mut monkies = (*input).clone();
  let multiple = find_multiple(&monkies, &vec![3]);
  for _ in 0..20 {
    do_round(&mut monkies, |v| (v % multiple) / 3);
  }
  compute_top_two(&monkies)
}

pub fn part2(input: &InputType) -> OutputType {
  let mut monkies = (*input).clone();
  let multiple = find_multiple(&monkies, &vec![]);
  for _ in 0..10_000 {
    do_round(&mut monkies, |v| v % multiple);
  }
  compute_top_two(&monkies)
}

#[cfg(test)]
mod tests {
  use crate::day11::{generator, part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!(10605, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(2713310158, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
}
