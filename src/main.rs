use argh::FromArgs;
use colored::Colorize;
use omalley_aoc2022::{FUNCS,INPUTS,NAMES};
use omalley_aoc2022::utils::{DayResult,time};
use serde::{Deserialize,Serialize};
use std::collections::BTreeMap;
use std::{fs, io};
use std::path::Path;

#[derive(FromArgs)]
/** Solution for Advent of Code (https://adventofcode.com/)*/
struct Args {
  /// the input directory
  #[argh(option, short='i')]
  input: Option<String>,

  /// a single day to execute (defaults to all)
  #[argh(positional)]
  day: Option<usize>,
}

#[derive(Default,Deserialize,Serialize)]
struct Answers {
  // map from day name to answers
  days: BTreeMap<String,Vec<String>>,
}

impl Answers {
  const FILENAME: &'static str = "answers.yml";

  fn read() -> Self {
    if let Ok(f) = fs::File::open(Self::FILENAME) {
      serde_yaml::from_reader(f).expect("Could not read answers")
    } else {
      Self::default()
    }
  }

  fn update(&mut self, delta_list: &Vec<DayResult>) {
    for delta in delta_list {
      let new_val = delta.get_answers();
      if let Some(prev) =
          self.days.insert(delta.day.to_string(), new_val.clone()) {
        if prev != new_val {
          println!("{}", format!("Output for {} changed from {:?} to {:?}!",
                                 delta.pretty_day(), prev, new_val).bold());
        }
      }
    }
  }

  fn write(&self) {
    let f = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(Self::FILENAME)
      .expect("Couldn't open file");
    serde_yaml::to_writer(f, self).unwrap();
  }
}

/// Read the data files from the in_dir into a vector of string.
fn read_inputs(in_dir: &str) -> io::Result<Vec<String>> {
  let data: Vec<io::Result<String>> = NAMES.iter()
    .map(|&day| {
      let filename = format!("{in_dir}/{day}.txt");
      fs::read_to_string(Path::new(&filename))
    })
    .collect();
  data.into_iter().collect()
}

fn main() {
    let args: Args = argh::from_env();
    // Did the user pick a single day to run
    let day_filter: Option<usize> = match args.day {
        Some(day) => {
            let name = format!("day{}", day);
            Some(NAMES.iter().position(|x| **x == name)
              .expect("Requested an unimplemented day"))
        },
        None => None
    };
    let inputs: Vec<String>;
    if let Some(in_dir) = &args.input {
      println!("{} {}\n", "Reading from".bold(), in_dir);
      inputs = read_inputs(&in_dir).expect("Can't read input dir")
    } else {
      inputs = INPUTS.iter().map(|&s| s.to_string()).collect()
    };

     let (elapsed, results) = time(&|| {
        crate::FUNCS.iter().enumerate()
          .filter(|(p, _)| day_filter.is_none() || day_filter.unwrap() == *p)
          .map(|(p, f)| f(&inputs[p]))
          .collect::<Vec<DayResult>>()
    });

    for r in &results {
      println!("{}", r);
    }
    println!("{} {}", "Overall runtime".bold(), format!("({:.2?})", elapsed).dimmed());

    let mut old_answers = Answers::read();
    old_answers.update(&results);
    old_answers.write();
}
