use priority_queue::PriorityQueue;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type InputType = Vec<Blueprint>;
type OutputType = usize;

const TIME: Count = 24;
const PART2_TIME: Count = 32;
const PLAN_LIMIT: usize = 3;

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

  fn get_priority(&self) -> usize {
    self.robots.iter().rev().fold(0, |acc, v| (acc << 16) | *v as usize)
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
    /*println!("{self:?}");
    for n in &result {
      println!(" --> {n:?}");
    }*/
    result
  }
}

fn best_score(blueprint: &Blueprint, time: Count) -> Count {
  let mut pending = PriorityQueue::new();
  pending.push(State::new(time), 0);
  let mut max = 0;
  while let Some((state, _)) = pending.pop() {
    if state.remaining_time == 0 {
      max = Count::max(max, state.stock[Resource::Geode.idx()]);
    } else {
      for next in state.next(blueprint).into_iter() {
        let priority = next.get_priority();
        pending.push(next, priority);
      }
    }
  }
  max
}

pub fn part1(input: &InputType) -> OutputType {
  input.iter()
      .map(|bp| bp.id * best_score(bp, TIME) as usize)
      .sum()
}

pub fn part2(input: &InputType) -> OutputType {
  input.iter().take(PLAN_LIMIT)
      .map(|bp| best_score(bp, PART2_TIME) as usize)
      .product()
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
