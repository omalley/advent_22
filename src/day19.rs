use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type InputType = Vec<Blueprint>;
type OutputType = usize;

const TIME: Count = 24;
const PART2_TIME: Count = 32;
const PART2_PLAN_LIMIT: usize = 3;

#[derive(Clone,Copy,Debug,EnumIter)]
enum Resource {
  Ore,
  Clay,
  Obsidian,
  Geode,
}

impl Resource {
  const SIZE: usize = 4;

  fn idx(&self) -> usize {
    *self as usize
  }
}

type Count = u16;

#[derive(Debug,Default)]
pub struct Blueprint {
  id: usize,
  robot: [[Count; Resource::SIZE]; Resource::SIZE],
  max_robots: [Count; Resource::SIZE],
}

impl Blueprint {
  fn parse(line: &str) -> Self {
    let words: Vec<&str> = line.split_whitespace().collect();
    let mut result = Blueprint::default();
    result.id = words[1][..words[1].len()-1].parse().unwrap();
    result.robot[Resource::Ore.idx()][Resource::Ore.idx()] = words[6].parse().unwrap();
    result.robot[Resource::Clay.idx()][Resource::Ore.idx()] = words[12].parse().unwrap();
    result.robot[Resource::Obsidian.idx()][Resource::Ore.idx()] = words[18].parse().unwrap();
    result.robot[Resource::Obsidian.idx()][Resource::Clay.idx()] = words[21].parse().unwrap();
    result.robot[Resource::Geode.idx()][Resource::Ore.idx()] = words[27].parse().unwrap();
    result.robot[Resource::Geode.idx()][Resource::Obsidian.idx()] = words[30].parse().unwrap();
    for r in Resource::iter() {
      result.max_robots[r.idx()] = result.robot.iter().map(|col| col[r.idx()]).max().unwrap();
    }
    result.max_robots[Resource::Geode.idx()] = Count::MAX;
    result
  }
}

pub fn generator(input: &str) -> InputType {
  input.lines().map(|l| Blueprint::parse(l)).collect()
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct State {
  remaining_time: Count,
  stock: [Count; Resource::SIZE],
  robots: [Count; Resource::SIZE],
}

impl State {
  fn new(time: Count) -> Self {
    let stock = [0; Resource::SIZE];
    let mut robots = [0; Resource::SIZE];
    robots[Resource::Ore.idx()] = 1;
    State{remaining_time: time, stock, robots}
  }

  /// What resources will we have at the end of the time if
  /// we don't build a robot?
  fn gather(&mut self, minutes: Count) {
    self.remaining_time -= minutes;
    for i in 0..Resource::SIZE {
      self.stock[i] += self.robots[i] * minutes;
    }
  }

  /// What is the state if we build the given robot type?
  fn can_build(&self, robot: Resource, blueprint: &Blueprint) -> Option<Self> {
    let robot_idx = robot.idx();
    // Figure out how long we need to wait to gather enough resources
    let mut idle_time = 0;
    for r in 0..Resource::SIZE {
      if self.stock[r] < blueprint.robot[robot_idx][r] {
        if self.robots[r] != 0 {
          let needed = blueprint.robot[robot_idx][r] - self.stock[r];
          idle_time = Count::max(idle_time, (needed + self.robots[r] - 1) / self.robots[r]);
        } else {
          return None
        }
      }
    }
    if idle_time >= self.remaining_time {
      return None
    }
    let mut new_state = self.clone();
    new_state.gather(idle_time + 1);
    for r in 0..Resource::SIZE {
      new_state.stock[r] -= blueprint.robot[robot_idx][r];
    }
    new_state.robots[robot_idx] += 1;
    Some(new_state)
  }

  /// Generate the next states from the current
  fn next(&self, blueprint: &Blueprint) -> Vec<Self> {
    let mut result = Vec::new();
    // Figure out which robot we should build next
    for robot in Resource::iter() {
      if self.remaining_time > 1 &&
          self.robots[robot.idx()] < blueprint.max_robots[robot.idx()] {
        if let Some(new_state) = self.can_build(robot, blueprint) {
          result.push(new_state);
        }
      }
    }
    // If we can't build something, just burn the rest of the time.
    if result.is_empty() {
      let mut sit_around = self.clone();
      sit_around.gather(self.remaining_time);
      result.push(sit_around);
    }
    result
  }

  fn gather_upper_limit(turns: Count) -> Count {
    turns * (turns + 1) / 2
  }

  fn turns_to_gather(resource: Count) -> Count {
    let mut remain = resource;
    let mut result = 1;
    while remain > result {
      remain -= result;
    }
    result + 1
  }

  /// What is the upper bound on how high a score we can get from here.
  fn limit(&self, blueprint: &Blueprint) -> Count {
    let mut turns_to_build = 0;
    for rbt in Resource::iter() {
      if self.robots[rbt.idx()] == 0 {
//        turns_to_build += 1 + Self::turns_to_gather(blueprint.robot[rbt.idx()][])
      }
    }
    0
  }
}

fn best_score(blueprint: &Blueprint, time: Count, state_count: &mut usize) -> Count {
  let mut pending = Vec::new();
  pending.push(State::new(time));
  let mut max = 0;
  while let Some(state) = pending.pop() {
    *state_count += 1;
    if state.remaining_time == 0 {
      let old_max = max;
      max = Count::max(max, state.stock[Resource::Geode.idx()]);
      if old_max != max {
        println!("max {old_max} to {max} at {state_count}");
      }
    } else {
      pending.extend_from_slice(&state.next(blueprint));
    }
  }
  max
}

pub fn part1(input: &InputType) -> OutputType {
  let mut state_count = 0;
  input.iter()
      .map(|bp| bp.id * best_score(bp, TIME, &mut state_count) as usize)
      .sum()
}

pub fn part2(input: &InputType) -> OutputType {
  let mut state_count = 0;
  let result =
  input.iter().take(PART2_PLAN_LIMIT)
      .map(|bp| best_score(bp, PART2_TIME, &mut state_count) as usize)
      .product();
  println!("part 2 = {state_count}");
  result
}

#[cfg(test)]
mod tests {
  use crate::day19::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(2, input.len());
    assert_eq!(33, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(3472, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"Blueprint 1: \
  Each ore robot costs 4 ore. \
  Each clay robot costs 2 ore. \
  Each obsidian robot costs 3 ore and 14 clay. \
  Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: \
  Each ore robot costs 2 ore. \
  Each clay robot costs 3 ore. \
  Each obsidian robot costs 3 ore and 8 clay. \
  Each geode robot costs 3 ore and 12 obsidian.\n";
}
