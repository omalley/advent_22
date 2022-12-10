type InputType = Vec<Command>;
type OutputType = i64;

#[derive(Debug)]
pub enum Command {
  ADDX(i64),
  NOOP,
}

impl Command {
  fn parse(line: &str) -> Self {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words[0] {
      "addx" => Self::ADDX(words[1].parse::<i64>().expect("expected delta")),
      "noop" => Self::NOOP,
      _ => panic!("Unknown command {}", line),
    }
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines()
    .map(|l| Command::parse(l))
    .collect()
}

struct State {
  prev: i64,
  x: i64,
  time: usize
}

impl State {
  fn default() -> Self {
    State{prev: 1, x: 1, time: 0}
  }

  fn execute(&mut self, cmd: &Command) {
    self.prev = self.x;
    match cmd {
      Command::ADDX(val) => { self.x += val; self.time += 2; },
      Command::NOOP => { self.time += 1; }
    }
  }
}

const FIRST_CHECK: usize = 20;
const PERIOD: usize = 40;

pub fn part1(input: &InputType) -> OutputType {
  let mut next_cycle = FIRST_CHECK;
  let mut state = State::default();
  let mut result: i64 = 0;
  for cmd in input {
    state.execute(cmd);
    if state.time >= next_cycle {
      result += state.prev * next_cycle as i64;
      next_cycle += PERIOD;
    }
  }
  result
}

pub fn part2(input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day10::{generator, part1, part2};

  const INPUT: &str =
"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

  #[test]
  fn test_part1() {
    assert_eq!(13140, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(1, part2(&generator(INPUT)));
  }
}
