use std::collections::{HashMap};

type InputType = Vec<Command>;
type OutputType = usize;

#[derive(Debug)]
pub enum ListObject {
  FILE(String, usize),
  DIR(String)
}

impl ListObject {
  fn parse(line: &str) -> Self {
    match line.split_once(" ").unwrap() {
      ("dir", name) => Self::DIR(name.to_string()),
      (num, name) =>
        Self::FILE(name.to_string(),num.parse::<usize>().expect("size")),
    }
  }

  fn size(&self) -> usize {
    match self {
      Self::FILE(_, size) => *size,
      _ => 0,
    }
  }
}

#[derive(Debug)]
pub enum Command {
  CD(String),
  LS(Vec<ListObject>),
}

impl Command {
  fn parse(cmd: &str) -> Self {
    let mut lines = cmd.lines();
    let words: Vec<&str> = lines.next().unwrap().split_whitespace().collect();
    match words[0] {
      "cd" => Self::CD(words[1].to_string()),
      "ls" => Self::LS(lines.map(|item| ListObject::parse(item)).collect()),
      _ => panic!("Unknown command {}", words[0]),
    }
  }
}

pub fn generator(input: &str) -> InputType {
  input[2..].split("\n$ ").map(|cmd| Command::parse(cmd)).collect()
}

fn build_sizes(commands: &Vec<Command>) -> HashMap<String, usize> {
  let mut cwd: Vec<String> = Vec::new();
  let mut result = HashMap::new();
  for cmd in commands {
    match cmd {
      Command::CD(path) => {
        match path.as_str() {
          "/" => {cwd.clear();},
          ".." => {cwd.pop();},
          _ => {cwd.push(path.clone());},
        }
      },
      Command::LS(contents) => {
        // get the size of all files in this directory
        let total: usize = contents.iter().map(|obj| obj.size()).sum();
        // add them to each directory up the tree
        for i in 0..(cwd.len()+1) {
          let path = cwd[0..i].join("/");
          result.insert(path.clone(), result.get(&path).unwrap_or(&0) + total);
        }
      },
    }
  }
  result
}

pub fn part1(input: &InputType) -> OutputType {
  let sizes = build_sizes(input);
  sizes.values().filter(|sz| **sz <= 100_000).sum()
}

pub fn part2(input: &InputType) -> OutputType {
  let sizes = build_sizes(input);
  let needed = (sizes.get("").unwrap_or(&0) + 30_000_000) - 70_000_000;
  let mut dir_size: Vec<usize> = sizes.values().cloned().collect();
  dir_size.sort();
  *dir_size.iter().find(|&sz| *sz >= needed).unwrap()
}

#[cfg(test)]
mod tests {
  use crate::day7::{generator, part1, part2};

  const INPUT: &str = "$ cd /\n\
                       $ ls\n\
                       dir a\n\
                       14848514 b.txt\n\
                       8504156 c.dat\n\
                       dir d\n\
                       $ cd a\n\
                       $ ls\n\
                       dir e\n\
                       29116 f\n\
                       2557 g\n\
                       62596 h.lst\n\
                       $ cd e\n\
                       $ ls\n\
                       584 i\n\
                       $ cd ..\n\
                       $ cd ..\n\
                       $ cd d\n\
                       $ ls\n\
                       4060174 j\n\
                       8033020 d.log\n\
                       5626152 d.ext\n\
                       7214296 k";

  #[test]
  fn test_generator() {
    let result = generator(INPUT);
    assert_eq!(10, result.len());
  }

  #[test]
  fn test_part1() {
    assert_eq!(95437, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(24933642, part2(&generator(INPUT)));
  }
}
