type InputType = Vec<Command>;
type OutputType = String;

#[derive(Debug)]
pub enum Command {
  AddX(i64),
  NoOp,
}

impl Command {
  fn parse(line: &str) -> Self {
    let words: Vec<&str> = line.split_whitespace().collect();
    match words[0] {
      "addx" => Self::AddX(words[1].parse::<i64>().expect("expected delta")),
      "noop" => Self::NoOp,
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
      Command::AddX(val) => { self.x += val; self.time += 2; },
      Command::NoOp => { self.time += 1; }
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
  format!("{}", result)
}

fn pixel(value: i64, column: usize) -> char {
  if i64::abs(value - column as i64) <= 1 {
    '#'
  } else {
    ' '
  }
}

pub fn part2(input: &InputType) -> OutputType {
  let mut result = String::new();
  let mut state = State::default();
  let mut time: usize = 0;
  for cmd in input {
    state.execute(cmd);
    while time < state.time {
      result.push(pixel(state.prev, time % PERIOD));
      time += 1;
      if time % PERIOD == 0 {
        result.push('\n');
      }
    }
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::day10::{generator, part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!("13140", part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    let expected = "##  ##  ##  ##  ##  ##  ##  ##  ##  ##  \n\
                           ###   ###   ###   ###   ###   ###   ### \n\
                           ####    ####    ####    ####    ####    \n\
                           #####     #####     #####     #####     \n\
                           ######      ######      ######      ####\n\
                           #######       #######       #######     \n".to_string();
    assert_eq!(expected, part2(&generator(INPUT)));
  }

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
}
