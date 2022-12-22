use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

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
  Equals,
  Human,
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

  fn evaluate(&self, left: Num, right: Num) -> Num {
    match self {
      Operation::Literal(n) => *n,
      Operation::Plus => left + right,
      Operation::Minus => left - right,
      Operation::Multiply => left * right,
      Operation::Divide => left / right,
      Operation::Equals => if left == right { 1 } else { 0 },
      _ => panic!("Can't handle operation {self:?}"),
    }
  }

  fn evaluate_for_left(&self, right: Num, result: Num) -> Num {
    match self {
      Operation::Plus => result - right,
      Operation::Minus => result + right,
      Operation::Multiply => result / right,
      Operation::Divide => result * right,
      _ => panic!("Can't handle operation {self:?}"),
    }
  }

  fn evaluate_for_right(&self, left: Num, result: Num) -> Num {
    match self {
      Operation::Plus => result - left,
      Operation::Minus => left - result,
      Operation::Multiply => result / left,
      Operation::Divide => left / result,
      _ => panic!("Can't handle operation {self:?}"),
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
      _ => panic!("Can't handle {:?}", results[monkey].op),
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

const HUMAN_NAME: &str = "humn";

#[derive(Clone,Debug)]
enum SymbolicState {
  Literal(Num),
  Input,
  Op(Operation, Rc<SymbolicState>, Rc<SymbolicState>)
}

impl SymbolicState {

  /// Do constant folding for operations where both parameters are literals.
  fn simplify(expr: SymbolicState) -> SymbolicState {
    match &expr {
      SymbolicState::Op(op, left, right) => {
        match (left.borrow(), right.borrow()) {
          (SymbolicState::Literal(l), SymbolicState::Literal(r)) => {
            return SymbolicState::Literal(op.evaluate(*l, *r));
          }
          _ => { },
        }
      },
      _ => { },
    }
    expr
  }

  fn evaluate(values: &mut [Option<Rc<SymbolicState>>], expr_idx: usize,
              ops: &[Operation], params: &[Vec<usize>]) -> Rc<SymbolicState> {
    let result = Rc::new(match ops[expr_idx] {
      Operation::Literal(n) => SymbolicState::Literal(n),
      Operation::Human => SymbolicState::Input,
      op => Self::simplify(SymbolicState::Op(op,
                                           Self::evaluate(values, params[expr_idx][0],
                                                          ops, params),
                                           Self::evaluate(values, params[expr_idx][1],
                                                          ops, params))),
    });
    values[expr_idx] = Some(result.clone());
    result
  }

  /// The expr must equal the given result, so simplify the expression as much
  /// as possible.
  fn reduce_equals(expr: Rc<SymbolicState>, result: Num) -> Rc<SymbolicState> {
    let mut cur_result = result;
    let mut cur_expr = expr;
    loop {
      match cur_expr.clone().borrow() {
        SymbolicState::Op(op, left, right) => {
          if let SymbolicState::Literal(n) = left.borrow() {
            cur_expr = right.clone();
            cur_result = op.evaluate_for_right(*n, cur_result);
          } else if let SymbolicState::Literal(n) = right.borrow() {
            cur_expr = left.clone();
            cur_result = op.evaluate_for_left(*n, cur_result);
          } else {
            break;
          }
        },
        _ => { break; }
      }
    }
    Rc::new(SymbolicState::Op(Operation::Equals, cur_expr,
                      Rc::new(SymbolicState::Literal(cur_result))))
  }

  fn reduce(expr: Rc<SymbolicState>) -> Rc<SymbolicState> {
    match expr.borrow() {
      SymbolicState::Op(Operation::Equals, left, right) => {
        if let SymbolicState::Literal(n) = left.borrow() {
          return Self::reduce_equals(right.clone(), *n);
        }
        if let SymbolicState::Literal(n) = right.borrow() {
          return Self::reduce_equals(left.clone(), *n);
        }
      },
      _ => {},
    }
    expr
  }
}

pub fn part2(input: &InputType) -> OutputType {
  let names = build_name_map(input);
  // Copy over the operations where we can change them with overrides
  let mut ops: Vec<Operation> = input.iter().map(|m| m.op).collect();
  ops[*names.get(HUMAN_NAME).unwrap()] = Operation::Human;
  ops[*names.get(ROOT_NAME).unwrap()] = Operation::Equals;
  // build the list of params
  let params: Vec<Vec<usize>> = input.iter()
    .map(|m| m.parameters.iter().map(|p| *names.get(p).unwrap())
      .collect::<Vec<usize>>())
    .collect();
  // Compute the symbolic values with a cache
  let mut values: Vec<Option<Rc<SymbolicState>>> = vec![None; input.len()];
  let ans = SymbolicState::reduce(SymbolicState::evaluate(&mut values,
                                    *names.get(ROOT_NAME).unwrap(),
                                    &ops, &params));
  if let SymbolicState::Op(Operation::Equals, _, right) = ans.borrow() {
    if let SymbolicState::Literal(n) = right.borrow() {
      return *n;
    }
  }
  panic!("Not simplified {ans:?}");
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
    assert_eq!(301, part2(&generator(INPUT)));
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
