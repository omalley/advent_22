type InputType = Vec<Num>;
type OutputType = Num;

type Num = i64;

pub fn generator(input: &str) -> InputType {
  input.lines().map(|l| l.parse::<Num>().unwrap()).collect()
}

#[derive(Default)]
struct DoubleLinkedListNode {
  value: Num,
  next: usize,
  prev: usize,
}

#[derive(Default)]
struct DoubleLinkedList {
  data: Vec<DoubleLinkedListNode>,
  start: usize,
  zero: Option<usize>,
}

impl DoubleLinkedList {

  /// Push to the back of the list
  fn push(&mut self, value: Num) {
    let mut node = DoubleLinkedListNode::default();
    if self.data.is_empty() {
      self.start = 0;
    } else {
      let head = self.start;
      let tail = self.data[head].prev;
      let new_posn = self.data.len();
      node.next = head;
      node.prev = tail;
      self.data[head].prev = new_posn;
      self.data[tail].next = new_posn;
    }
    if value == 0 {
      self.zero = Some(self.data.len());
    } else {
      node.value = value;
    }
    self.data.push(node);
  }

  fn seek(&self, node: usize, count: Num) -> usize {
    let mut ptr = node;
    if count > 0 {
      for _ in 0..count {
        ptr = self.data[ptr].next;
      }
    } else if count < 0 {
      for _ in count..=0 {
        ptr = self.data[ptr].prev;
      }
    }
    ptr
  }

  fn process(&mut self, node: usize) {
    let value = (self.data[node].value) % (self.data.len() - 1) as Num;
    if value != 0 {
      // Get the new before and after nodes
      let new_prev = self.seek(node, value);
      let new_next = self.data[new_prev].next;
      // Get the old before and after nodes
      let old_prev = self.data[node].prev;
      let old_next = self.data[node].next;
      // Fix up all of the links
      self.data[new_prev].next = node;
      self.data[new_next].prev = node;
      self.data[node].next = new_next;
      self.data[node].prev = new_prev;
      self.data[old_prev].next = old_next;
      self.data[old_next].prev = old_prev;
    }
  }

  fn shuffle(&mut self) {
    for ptr in 0..self.data.len() {
      self.process(ptr);
    }
  }

  /// Find the given nodes relative to the 0 node.
  fn find_nodes(&self, targets: &[Num]) -> Vec<Num> {
    let mut result = Vec::new();
    if self.zero.is_none() {
      return result;
    }
    let list_size = self.data.len() as Num;
    let mut sorted_targets: Vec<Num> = targets.iter()
        .map(|&p| p.rem_euclid(list_size))
        .collect();
    sorted_targets.sort_unstable();
    let mut current = self.zero.unwrap();
    let mut current_pos: Num = 0;
    for target in sorted_targets {
      current = self.seek(current, target - current_pos);
      result.push(self.data[current].value);
      current_pos = target;
    }
    result
  }
}

pub fn part1(input: &InputType) -> OutputType {
  let mut list = DoubleLinkedList::default();
  for n in input {
    list.push(*n);
  }
  list.shuffle();
  let answer =   list.find_nodes(&[1000, 2000, 3000]);
  answer.iter().sum()
}

const DECRYPTION_KEY: Num = 811589153;
const ITERATIONS: usize = 10;

pub fn part2(input: &InputType) -> OutputType {
  let mut list = DoubleLinkedList::default();
  for n in input {
    list.push(*n * DECRYPTION_KEY);
  }
  for _ in 0..ITERATIONS {
    list.shuffle();
  }
  list.find_nodes(&[1000, 2000, 3000]).iter().sum()
}

#[cfg(test)]
mod tests {
  use crate::day20::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(7, input.len());
    assert_eq!(3, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(1623178306, part2(&generator(INPUT)));
  }

  const INPUT: &str = "1\n\
                       2\n\
                       -3\n\
                       3\n\
                       -2\n\
                       0\n\
                       4";
}
