use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::rc::Rc;

type InputType = Vec<Num>;
type OutputType = Num;

type Num = i64;

pub fn generator(input: &str) -> InputType {
  input.lines().map(|l| l.parse::<Num>().unwrap()).collect()
}

type Link = Option<Rc<RefCell<DoubleLinkedListNode>>>;

struct DoubleLinkedListNode {
  next: Link,
  prev: Link,
  original: Link,
  value: Num,
}

impl DoubleLinkedListNode {
  fn new(value: Num) -> Self {
    DoubleLinkedListNode{next: None, prev: None, original: None, value}
  }
}

#[derive(Default)]
struct DoubleLinkedList {
  start: Link,
  zero: Link,
  size: usize,
}

impl DoubleLinkedList {

  /// Push to the back of the list
  fn push(&mut self, value: Num) {
    self.size += 1;
    let node = Rc::new(RefCell::new(DoubleLinkedListNode::new(value)));
    if value == 0 {
      self.zero = Some(node.clone());
    }
    match &self.start {
      Some(head) => {
        let tail = &head.borrow().prev.as_ref().unwrap().clone();
        node.borrow_mut().next = Some(head.clone());
        node.borrow_mut().prev = Some(tail.clone());
        head.borrow_mut().prev = Some(node.clone());
        tail.borrow_mut().original = Some(node.clone());
        tail.borrow_mut().next = Some(node.clone());
      },
      None => {
        node.borrow_mut().prev = Some(node.clone());
        node.borrow_mut().next = Some(node.clone());
        self.start = Some(node.clone());
      },
    }
  }

  fn seek(node: &Rc<RefCell<DoubleLinkedListNode>>, count: Num) -> Rc<RefCell<DoubleLinkedListNode>> {
    let mut ptr = node.clone();
    if count > 0 {
      for _ in 0..count {
        let n = ptr.borrow().next.as_ref().unwrap().clone();
        ptr = n;
      }
    } else if count < 0 {
      for _ in count..=0 {
        let n = ptr.borrow().prev.as_ref().unwrap().clone();
        ptr = n;
      }
    }
    ptr
  }

  fn process(&mut self, node: Rc<RefCell<DoubleLinkedListNode>>) {
    let value = (node.borrow().value) % (self.size - 1) as Num;
    if value != 0 {
      // Get the new before and after nodes
      let new_prev = Self::seek(&node, value);
      let new_next = new_prev.borrow().next.as_ref().unwrap().clone();
      // Get the old before and after nodes
      let old_prev = node.borrow().prev.as_ref().unwrap().clone();
      let old_next = node.borrow().next.as_ref().unwrap().clone();
      // Fix up all of the links
      new_prev.borrow_mut().next = Some(node.clone());
      new_next.borrow_mut().prev = Some(node.clone());
      node.borrow_mut().next = Some(new_next.clone());
      node.borrow_mut().prev = Some(new_prev.clone());
      old_prev.borrow_mut().next = Some(old_next.clone());
      old_next.borrow_mut().prev = Some(old_prev.clone());
    }
  }

  fn shuffle(&mut self) {
    if let Some(head) = &self.start {
      let mut ptr = head.clone();
      loop {
        self.process(ptr.clone());
        if ptr.borrow().original.is_none() {
          break
        } else {
          let next = ptr.borrow().original.as_ref().unwrap().clone();
          ptr = next;
        }
      }
    }
  }

  /// Find the given nodes relative to the 0 node.
  fn find_nodes(&self, targets: &[Num]) -> Vec<Num> {
    let mut result = Vec::new();
    if self.zero.is_none() {
      return result;
    }
    let list_size = self.size as Num;
    let mut sorted_targets: Vec<Num> = targets.iter()
        .map(|p| ((p % list_size) + list_size) % list_size)
        .collect();
    sorted_targets.sort_unstable();
    let mut current = self.zero.as_ref().unwrap().clone();
    let mut current_pos: Num = 0;
    for target in sorted_targets {
      current = Self::seek(&current, target - current_pos);
      result.push(current.borrow().value);
      current_pos = target;
    }
    result
  }
}

impl Display for DoubleLinkedList {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "[")?;
    if let Some(head) = &self.start {
      let mut ptr = head.clone();
      loop {
        write!(f, "{}", ptr.borrow().value)?;
        let next = ptr.borrow().next.as_ref().unwrap().clone();
        ptr = next;
        if Rc::ptr_eq(&ptr, head) {
          break;
        } else {
          write!(f, ",")?;
        }
      }
    }
    write!(f, "]")
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
