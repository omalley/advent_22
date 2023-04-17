use std::collections::HashMap;

type InputType = Caves;
type OutputType = u64;

const TIME_LIMIT: u64 = 30;
const PART2_TIME: u64 = 26;
const PART2_WORKERS: usize = 2;

#[derive(Debug)]
pub struct Valve {
  name: String,
  flow: u64,
  next: Vec<String>,
}

impl Valve {
  fn parse(line: &str) -> Self {
    let words: Vec<&str> = line.split_whitespace().collect();
    let name = words[1].to_string();
    // remove "rate=" and ";"
    let mut flow_str = words[4][5..].to_string();
    flow_str.pop();
    let flow = flow_str.parse::<u64>().unwrap();
    let next: Vec<String> = words[9..].join(" ").split(", ").map(|s| s.to_string()).collect();
    Valve{name, flow, next}
  }
}

#[derive(Debug)]
pub struct Caves {
  start: usize,
  flows: Vec<u64>,
  /// Distance[from][dest]
  distances: Vec<Vec<u8>>,
}

impl Caves {
  fn parse(input: &str) -> Self {
    let valves: Vec<Valve> = input.lines().map(|l| Valve::parse(l)).collect();
    let mut map: HashMap<&str, usize> = HashMap::new();
    let mut flows: Vec<u64> = vec![0; valves.len()];
    let mut distances = vec![vec![valves.len() as u8; valves.len()]; valves.len()];
    for (i, v) in valves.iter().enumerate() {
      map.insert(&v.name, i);
      flows[i] = v.flow;
    }
    let start = *map.get("AA").unwrap();
    // Set the distance to 1 for every direct connection
    for (from, v) in valves.iter().enumerate() {
      distances[from][from] = 0;
      for dest in v.next.iter().map(|n| *map.get(n.as_str()).unwrap()) {
        distances[from][dest] = 1;
      }
    }
    // compute the shortest distances for all pairs
    for i in 0..distances.len() {
      for j in 0..distances.len() {
        for k in 0..distances.len() {
          distances[j][k] = u8::min(distances[j][k], distances[j][i] + distances[i][k]);
        }
      }
    }
    Caves{start, flows, distances}
  }

  /// Get the u64/bitmap of the closed valves
  fn get_closed(&self) -> u64 {
    self.flows.iter().enumerate()
      .filter(|(_, &f)| f > 0)
      .fold(0, |acc, (i, _)| acc | (1 << i))
  }
}

pub fn generator(input: &str) -> InputType {
  Caves::parse(input)
}

/// Define the interface for this problem's state types.
trait State {
  /// The type itself, so that we don't need to box the results
  /// of next.
  type Data: State;
  /// What is the score for this state?
  fn score(&self) -> u64;
  /// Which states are reachable from this one?
  fn next(&self, caves: &Caves) -> Vec<Self::Data>;
}

#[derive(Clone,Debug)]
pub struct Part1 {
  /// bit map of the valves that are still closed
  shut: u64,
  location: usize,
  remaining_time: u64,
  total_flow: u64,
}

impl Part1 {
  fn new(input: &Caves) -> Self {
    Part1 {
      shut: input.get_closed(),
      location: input.start,
      remaining_time: TIME_LIMIT,
      total_flow: 0,
    }
  }

  fn move_to(&self, dest: usize, caves: &Caves) -> Self {
    let shut = self.shut & !(1 << dest);
    let remaining_time = self.remaining_time - caves.distances[self.location][dest] as u64 - 1;
    let total_flow = self.total_flow + remaining_time * caves.flows[dest];
    Part1 { location: dest, shut, remaining_time, total_flow }
  }
}

impl State for Part1 {
  type Data = Part1;

  fn score(&self) -> u64 {
    self.total_flow
  }

  fn next(&self, caves: &Caves) -> Vec<Self> {
    let mut result = Vec::new();
    let mut closed_valves = self.shut;
    while closed_valves != 0 {
      let zeros = closed_valves.trailing_zeros() as usize;
      if caves.distances[self.location][zeros] as u64 + 1 < self.remaining_time {
        result.push(self.move_to(zeros, caves));
      }
      closed_valves &= !(1 << zeros);
    }
    result
  }
}

/// Generic routine that searches for the highest score from
/// the given initial state.
fn search<T: State<Data=T>>(caves: &Caves, initial: T) -> OutputType {
  let mut queue = Vec::new();
  queue.push(initial);
  let mut max = 0;
  while let Some(state) = queue.pop() {
    max = max.max(state.score());
    let next: Vec<T> = state.next(caves);
    if !next.is_empty() {
      queue.extend(next.into_iter());
    }
  }
  max
}

pub fn part1(input: &InputType) -> OutputType {
  search(input, Part1::new(input))
}

/// Now we have 2 workers (us and the elephant), so we
/// need to track both.
#[derive(Clone,Debug)]
struct Part2 {
  /// bit map of the valves that are still closed
  shut: u64,
  locations: [usize; PART2_WORKERS],
  remaining_times: [u64; PART2_WORKERS],
  total_flow: u64,
}

impl Part2 {
  fn new(input: &Caves) -> Self {
    Part2 {
      shut: input.get_closed(),
      locations: [input.start; PART2_WORKERS],
      remaining_times: [PART2_TIME; PART2_WORKERS],
      total_flow: 0,
    }
  }

  fn move_to(&self, worker: usize, dest: usize, caves: &Caves) -> Self {
    let shut = self.shut & !(1 << dest);
    let mut locations = self.locations;
    locations[worker] = dest;
    let mut remaining_times = self.remaining_times;
    remaining_times[worker] -= caves.distances[self.locations[worker]][dest] as u64 + 1;
    let total_flow = self.total_flow + remaining_times[worker] * caves.flows[dest];
    Part2 { locations, shut, remaining_times, total_flow }
  }
}

impl State for Part2 {
  type Data = Part2;

  fn score(&self) -> u64 {
    self.total_flow
  }

  fn next(&self, caves: &Caves) -> Vec<Self> {
    let mut result = Vec::new();
    let worker = self.remaining_times.iter().enumerate()
      .fold((0, 0),
            |acc, (i, &v)| if v > acc.1 { (i, v) } else { acc }).0;
    let mut closed_valves = self.shut;
    while closed_valves != 0 {
      let zeros = closed_valves.trailing_zeros() as usize;
      if caves.distances[self.locations[worker]][zeros] as u64 + 1 < self.remaining_times[worker] {
        result.push(self.move_to(worker, zeros, caves));
      }
      closed_valves &= !(1 << zeros);
    }
    result
  }
}

pub fn part2(input: &InputType) -> OutputType {
  search(input, Part2::new(input))
}

#[cfg(test)]
mod tests {
  use crate::day16::{generator, part1, part2};

  #[test]
  fn test_part1() {
    assert_eq!(1651, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(1707, part2(&generator(INPUT)));
  }

  const INPUT: &str =
"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
}
