type InputType = CrossSection;
type OutputType = usize;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn parse(input: &str) -> Self {
    let (x,y) = input.split_once(",").unwrap();
    Point{x: x.parse().unwrap(), y: y.parse().unwrap()}
  }
}

#[derive(Debug)]
struct Ledges {
  lines: Vec<Vec<Point>>,
}

impl Ledges {
  fn parse(input: &str) -> Self {
    let lines = input.lines()
      .map(|s| s.split(" -> ")
        .map(|p| Point::parse(p)).collect())
      .collect();
    Ledges{lines}
  }

  fn left(&self) -> usize {
    self.lines.iter()
      .map(|r| r.iter().map(|p| p.x).min().unwrap_or(0))
      .min().unwrap_or(0)
  }

  fn right(&self) -> usize {
    self.lines.iter()
      .map(|r| r.iter().map(|p| p.x).max().unwrap_or(0))
      .max().unwrap_or(0)
  }

  fn height(&self) -> usize {
    self.lines.iter()
      .map(|r| r.iter().map(|p| p.y).max().unwrap_or(0))
      .max().unwrap_or(0)
  }
}

#[derive(Debug)]
pub struct CrossSection {
  ledges: Vec<Vec<bool>>,
  left: usize,
  right: usize,
  height: usize,
}

impl CrossSection {
  fn new(input: &Ledges) -> Self {
    let left = input.left() - 1;
    let right = input.right() + 1;
    let height = input.height() + 1;
    let mut ledges = vec![vec![false; right - left + 1]; height + 1];
    for row in &input.lines {
      for points in row.windows(2) {
        let mut p = points[0].clone();
        let delta_x = i64::signum(points[1].x as i64 - points[0].x as i64);
        let delta_y = i64::signum(points[1].y as i64 - points[0].y as i64);
        loop {
          ledges[p.y][p.x - left] = true;
          if p == points[1] {
            break
          }
          p.x = (p.x as i64 + delta_x) as usize;
          p.y = (p.y as i64 + delta_y) as usize;
        }
      }
    }
    CrossSection{ledges, left, right, height}
  }
}

pub fn generator(input: &str) -> InputType {
  let ledges = Ledges::parse(input);
  CrossSection::new(&ledges)
}

const START: Point = Point{x: 500, y: 0};

pub fn part1(input: &InputType) -> OutputType {
  let mut filled = input.ledges.clone();
  'grain: for grain in 0..usize::MAX {
    let mut x = START.x;
    for y in START.y..input.height {
      if filled[y+1][x - input.left] {
        if filled[y+1][x - 1 - input.left] {
          if filled[y+1][x + 1 - input.left] {
            filled[y][x - input.left] = true;
            continue 'grain;
          } else {
            x += 1;
          }
        } else {
         x -= 1;
        }
      }
    }
    return grain
  }
  usize::MAX
}

pub fn part2(input: &InputType) -> OutputType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day14::{generator, CrossSection, part1, part2};


  #[test]
  fn test_part1() {
    assert_eq!(24, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(93, part2(&generator(INPUT)));
  }

  const INPUT: &str = "498,4 -> 498,6 -> 496,6\n\
                       503,4 -> 502,4 -> 502,9 -> 494,9";
}
