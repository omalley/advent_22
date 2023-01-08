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
    for r in (0..Resource::SIZE).rev() {
      if self.stock[r] < blueprint.robot[robot_idx][r] {
        if self.robots[r] > 0 {
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

  /// Compute the potential maximum number of a resource that could be gathered in a given
  /// number of turns assuming that another robot is built every turn.
  fn estimate_resource(&self, resource: Resource, turns: Count) -> Count {
    let r = resource.idx();
    self.stock[r] + Count::max(1, self.robots[r]) * turns + turns * (turns - 1) / 2
  }

  /// Compute the minimum number of turns to gather a given number of a resource.
  /// Assumes that new robots are built 1 per a round.
  /// This function is the inverse of estimate_resource.
  fn estimate_turns(&self, resource: Resource, needed: Count) -> Count {
    let r = resource.idx();
    let robot = Count::max(1, self.robots[r]);
    if self.stock[r] >= needed {
      return 0
    }
    // res = s + robot * t + t * (t - 1) / 2
    // t^2/2 - t/2 + robot * t + s - res = 0
    // t^2 + (2 * robot - 1) * t + 2 * (s - res)  = 0
    let neg_b = 1.0 - 2.0 * f64::from(robot);
    let t = (neg_b + f64::sqrt(neg_b * neg_b + 8.0 * f64::from(needed - self.stock[r]))) / 2.0;
    let result = Count::try_from(f64::ceil(t) as i64).unwrap_or(Count::MAX);
    result
  }

  /// Compute the minimum number of turns to build the given kind of robot.
  fn estimate_robot_build(&self, robot: Resource, blueprint: &Blueprint) -> Count {
    let rbt = robot.idx();
    if self.robots[rbt] > 0 {
      return 0
    } else {
      Resource::iter()
        .map(|r|
          self.estimate_turns(r,
                              blueprint.robot[rbt][r.idx()]))
        .max().unwrap_or(0) + 1
    }
  }

  /// What is the upper bound on how high a score we can get from here.
  fn limit(&self, blueprint: &Blueprint) -> Count {
    let all_robots_built: Count = Resource::iter()
      .map(|r| self.estimate_robot_build(r, blueprint))
      .sum();
    if all_robots_built >= self.remaining_time {
      return self.stock[Resource::Geode.idx()]
    } else {
      self.estimate_resource(Resource::Geode,
                             self.remaining_time - all_robots_built)
    }
  }
}

fn best_score(blueprint: &Blueprint, time: Count) -> Count {
  let mut pending = Vec::new();
  pending.push(State::new(time));
  let mut max = 0;
  while let Some(state) = pending.pop() {
    max = Count::max(max, state.stock[Resource::Geode.idx()]);
    if state.remaining_time > 0 {
      pending.extend(state.next(blueprint).into_iter()
        .filter(|s| s.limit(blueprint) > max));
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
  input.iter().take(PART2_PLAN_LIMIT)
      .map(|bp| best_score(bp, PART2_TIME) as usize)
      .product()
}

#[cfg(test)]
mod tests {
  use crate::day19::{generator, part1, part2, Resource, State};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(2, input.len());
    assert_eq!(33, part1(&input));
  }

  #[test]
  fn test_estimate() {
    let state1 = State{remaining_time: 10,
                       stock: [16, 8, 4, 2],
                       robots: [4, 3, 2, 1]};
    // check resource collecting
    assert_eq!(3, state1.estimate_resource(Resource::Geode, 1));
    assert_eq!(5, state1.estimate_resource(Resource::Geode, 2));
    assert_eq!(8, state1.estimate_resource(Resource::Geode, 3));
    assert_eq!(12, state1.estimate_resource(Resource::Geode, 4));
    // test inverse
    assert_eq!(0, state1.estimate_turns(Resource::Geode, 1));
    assert_eq!(1, state1.estimate_turns(Resource::Geode, 3));
    assert_eq!(2, state1.estimate_turns(Resource::Geode, 5));
    assert_eq!(3, state1.estimate_turns(Resource::Geode, 8));
    assert_eq!(4, state1.estimate_turns(Resource::Geode, 9));
    assert_eq!(4, state1.estimate_turns(Resource::Geode, 11));
    assert_eq!(4, state1.estimate_turns(Resource::Geode, 12));
  }

  #[test]
  fn test_robot_estimate() {
    let bp = generator(INPUT).remove(0);
    let state = State::new(18);
    assert_eq!(0, state.estimate_robot_build(Resource::Ore, &bp));
    assert_eq!(3, state.estimate_robot_build(Resource::Clay, &bp));
    assert_eq!(6, state.estimate_robot_build(Resource::Obsidian, &bp));
    assert_eq!(5, state.estimate_robot_build(Resource::Geode, &bp));
    assert_eq!(10, state.limit(&bp));
    let state = State{remaining_time: 1, stock: [12, 19, 12, 9], robots: [3, 5, 7, 3]};
    assert_eq!(12, state.limit(&bp));
    let state = State{remaining_time: 0, stock: [15, 32, 6, 12], robots: [3, 5, 6, 4]};
    assert_eq!(12, state.limit(&bp));
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
