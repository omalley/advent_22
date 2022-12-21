use std::collections::HashMap;

type InputType = Vec<Monkey>;
type OutputType = Num;

type Num = i64;

#[derive(Clone,Copy,Debug)]
enum Operation {
  Literal(Num),
  Plus,
  Minus,
  Multiply,
  Divide,
}

impl Operation {
  fn parse(s: &str) -> Self {
    match s {
      "+" => Operation::Plus,
      "-" => Operation::Minus,
      "*" => Operation::Multiply,
      "/" => Operation::Divide,
      _ => panic!("Unknown character '{s}'"),
    }
  }
}

#[derive(Debug)]
pub struct Monkey {
  name: String,
  op: Operation,
  parameters: Vec<String>,
}

impl Monkey {
  fn parse(line: &str) -> Self {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut name = words[0].to_string();
    // remove the colon
    name.pop();
    let op: Operation;
    let parameters: Vec<String>;
    if words.len() == 4 {
      op = Operation::parse(&words[2]);
      parameters = vec![words[1].to_string(), words[3].to_string()];
    } else {
      op = Operation::Literal(words[1].parse::<Num>().unwrap());
      parameters = Vec::new();
    }
    Monkey{name, op, parameters}
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines().map(|l| Monkey::parse(l)).collect()
}

#[derive(Debug)]
struct Calculation {
  op: Operation,
  parameters: Vec<usize>,
  cache: Option<Num>,
}

impl Calculation {
  fn new(monkey: &Monkey, names: &HashMap<String, usize>) -> Self {
    match monkey.op {
      Operation::Literal(n) => Calculation{op: monkey.op, cache: Some(n), parameters: Vec::new()},
      _ => Calculation{op: monkey.op, cache: None,
          parameters: monkey.parameters.iter().map(|p| *names.get(p).unwrap()).collect()},
    }
  }

  fn evaluate(results: &mut [Calculation], monkey: usize) -> Num {
    if let Some(n) = results[monkey].cache {
      return n;
    }
    let result;
    match results[monkey].op {
      Operation::Literal(num) => { result = num; },
      Operation::Plus => {
        result = Self::evaluate(results, results[monkey].parameters[0]) +
            Self::evaluate(results, results[monkey].parameters[1])},
      Operation::Minus => {
        result = Self::evaluate(results, results[monkey].parameters[0]) -
            Self::evaluate(results, results[monkey].parameters[1])},
      Operation::Multiply => {
        result = Self::evaluate(results, results[monkey].parameters[0]) *
            Self::evaluate(results, results[monkey].parameters[1])},
      Operation::Divide => {
        result = Self::evaluate(results, results[monkey].parameters[0]) /
            Self::evaluate(results, results[monkey].parameters[1])},
    }
    results[monkey].cache = Some(result);
    result
  }
}

fn build_name_map(input: &[Monkey]) -> HashMap<String, usize> {
  let mut names: HashMap<String, usize> = HashMap::new();
  for i in 0..input.len() {
    names.insert(input[i].name.clone(), i);
  }
  names
}

const ROOT_NAME: &str = "root";

pub fn part1(input: &InputType) -> OutputType {
  let names = build_name_map(input);
  let mut calcs: Vec<Calculation> = input.iter()
      .map(|m| Calculation::new(m, &names))
      .collect();
  Calculation::evaluate(&mut calcs, *names.get(ROOT_NAME).unwrap())
}

pub fn part2(_input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day21::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(152, part1(&input));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(1623178306, part2(&generator(INPUT)));
  }

  const INPUT: &str = "root: pppw + sjmn\n\
                       dbpl: 5\n\
                       cczh: sllz + lgvd\n\
                       zczc: 2\n\
                       ptdq: humn - dvpt\n\
                       dvpt: 3\n\
                       lfqf: 4\n\
                       humn: 5\n\
                       ljgn: 2\n\
                       sjmn: drzm * dbpl\n\
                       sllz: 4\n\
                       pppw: cczh / lfqf\n\
                       lgvd: ljgn * ptdq\n\
                       drzm: hmdt - zczc\n\
                       hmdt: 32";
}
