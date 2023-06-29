use argh::FromArgs;
use colored::Colorize;
use omalley_aoc2022::{DayResult,FUNCS,INPUTS,NAMES,time};

#[derive(FromArgs)]
/** Solution for Advent of Code (https://adventofcode.com/)*/
struct Args {
  /// A single day to execute (all days by default)
  #[argh(option, short = 'd')]
  day: Option<usize>,
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

    let (elapsed, results) = time(&|| {
        crate::FUNCS.iter().enumerate()
          .filter(|(p, _)| day_filter.is_none() || day_filter.unwrap() == *p)
          .map(|(p, f)| f(INPUTS[p]))
          .collect::<Vec<DayResult>>()
    });

    for r in results {
        println!("{}", r);
    }
    println!("{} {}", "Overall runtime".bold(), format!("({:.2?})", elapsed).dimmed());
}
