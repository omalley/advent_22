#[derive(Clone,Debug)]
struct State {
  stacks: Vec<Vec<char>>,
}

impl State {
  fn parse(s: &str) -> Self {
    // convert each line to a list of characters
    let lines: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect::<Vec<char>>()).collect();
    // Read from the bottom & skip the stack numbers
    let mut itr = lines.iter().rev();
    itr.next();
    let mut stacks = Vec::new();
    for line in itr {
      let mut x = 0;
      // ignore the padding between columns
      while 4 * x + 1 < line.len() {
        if x >= stacks.len() {
          stacks.push(Vec::new());
        }
        let char = line[4 * x + 1];
        if char != ' ' {
          stacks.get_mut(x).unwrap().push(char);
        }
        x += 1;
      }
    }
    State{stacks}
  }

  fn get_top(&self) -> String {
    self.stacks.iter().filter_map(|s| s.last()).collect()
  }

  fn do_moves(&mut self, cmds: &[Move]) -> &mut Self {
    for cmd in cmds {
      for _ in 0..cmd.num_to_move {
        let ch = self.stacks[cmd.from].pop().unwrap();
        self.stacks[cmd.to].push(ch);
      }
    }
    self
  }

  fn do_moves_together(&mut self, cmds: &[Move]) -> &mut Self {
    for cmd in cmds {
      let from_posn = self.stacks[cmd.from].len() - cmd.num_to_move;
      let moving: Vec<char> = self.stacks[cmd.from].drain(from_posn..).collect();
      self.stacks[cmd.to].extend(moving.iter());
    }
    self
  }
}

#[derive(Debug)]
pub struct Move {
  num_to_move: usize,
  from: usize,
  to: usize,
}

impl Move {
  fn parse(s: &str) -> Self {
    let parts: Vec<&str> = s.split_whitespace().collect();
    Move{num_to_move: parts[1].parse().expect("num"),
      from: parts[3].parse::<usize>().expect("from") - 1,
      to:parts[5].parse::<usize>().expect("to") - 1}
  }
}

#[derive(Debug)]
pub struct InputType {
  state: State,
  moves: Vec<Move>,
}

pub fn generator(input: &str) -> InputType {
  let (state, moves) = input.split_once("\n\n").unwrap();
  InputType{
    state: State::parse(state),
    moves: moves.lines().map(|l| Move::parse(l)).collect(),
  }
}

pub fn part1(input: &InputType) -> String {
  input.state.clone().do_moves(&input.moves).get_top()
}

pub fn part2(input: &InputType) -> String {
  input.state.clone().do_moves_together(&input.moves).get_top()
}

#[cfg(test)]
mod tests {
  use crate::day5::{generator, part1, part2};

  const INPUT: &str = "    [D]\n\
                       [N] [C]\n\
                       [Z] [M] [P]\n\
                        1   2   3\n\
                       \n\
                       move 1 from 2 to 1\n\
                       move 3 from 1 to 3\n\
                       move 2 from 2 to 1\n\
                       move 1 from 1 to 2";

  #[test]
  fn parsing_test() {
    let input = generator(INPUT);
    assert_eq!("NDP", input.state.get_top());
  }

  #[test]
  fn test_part1() {
    assert_eq!("CMZ", part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!("MCD", part2(&generator(INPUT)));
  }
}
