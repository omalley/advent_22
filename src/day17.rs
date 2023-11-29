use std::collections::HashSet;

type InputType = Vec<Wind>;
type OutputType = usize;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Wind {
  Left,
  Right,
}

impl Wind {
  fn parse(ch: char) -> Self {
    match ch {
      '<' => Wind::Left,
      '>' => Wind::Right,
      _ => panic!("Unknown character '{ch}'"),
    }
  }

  /// What would be the new x given the range 0..upper?
  fn blow(&self, x: usize, upper: usize) -> Option<usize> {
    match self {
      Wind::Left => if x > 0 { Some(x - 1) } else { None },
      Wind::Right => if x < upper - 1 { Some(x + 1) } else { None },
    }
  }
}

pub fn generator(input: &str) -> InputType {
  input.chars().filter(|c| !c.is_ascii_whitespace())
    .map(Wind::parse).collect()
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum PieceKind {
  Bar,
  Plus,
  L,
  I,
  Square,
}

impl PieceKind {
  /// Return the bitmask in the low bits
  fn shape(&self) -> &[u8] {
    match self {
      PieceKind::Bar => &[0xf],
      PieceKind::Plus => &[0x2, 0x7, 0x2],
      PieceKind::L => &[0x7, 0x4, 0x4],
      PieceKind::I => &[0x1; 4],
      PieceKind::Square => &[0x3; 2],
    }
  }

  fn size(&self) -> (usize,usize) {
    match self {
      PieceKind::Bar => (4, 1),
      PieceKind::Plus | PieceKind::L => (3, 3),
      PieceKind::I => (1, 4),
      PieceKind::Square => (2, 2),
    }
  }
}

const INITIAL_X: usize = 2;
const BOARD_WIDTH: usize = 7;

#[derive(Debug)]
struct Piece {
  /// width, height
  size: (usize, usize),
  /// a static list of the mask at each horizontal position.
  masks: Vec<Vec<u8>>,
  /// positions from the left wall
  x: usize,
  /// position from the bottom
  y: usize,
}

impl Piece {
  fn new(kind: PieceKind) -> Self {
    let mut masks: Vec<Vec<u8>> = Vec::new();
    let size = kind.size();
    let base_mask = kind.shape();
    // Generate each shifted mask
    for x in 0..=(BOARD_WIDTH-size.0) {
      masks.push(base_mask.iter().map(|&v| v << x).collect());
    }
    Piece{size, masks, x:0, y:0}
  }
  fn reset(&mut self, y: usize) {
    self.x = INITIAL_X;
    self.y = y;
  }

  /// Does this piece overlap the given part of the board
  fn not_blocked(&self, x: usize, board: &[u8]) -> bool {
    !self.masks[x].iter().zip(board.iter()).any(|(&l,&r) | l & r != 0)
  }
}

const NEW_PIECE_HEIGHT: usize = 3;

struct Board {
  /// bitmap of the
  filled: Vec<u8>,
  /// The level of the first empty row
  current_height: usize,
}

impl Board {
  fn new(height: usize) -> Self {
    Board{filled: vec![0; height], current_height: 0}
  }

  /// If the piece can be blown in the given direction, do so.
  fn blow(&self, piece: &mut Piece, wind: Wind) {
    if let Some(new_x) = wind.blow(piece.x, BOARD_WIDTH + 1 - piece.size.0) {
      if piece.not_blocked(new_x, &self.filled[piece.y..piece.y + piece.size.1]) {
        piece.x = new_x;
      }
    }
  }

  /// Can the given piece fall another level?
  fn can_fall(&self, piece: &Piece) -> bool {
    piece.y > 0 && piece.not_blocked( piece.x,
                                      &self.filled[piece.y - 1..piece.y - 1 + piece.size.1])
  }

  /// Put a piece into its final place on the board
  fn place_piece(&mut self, piece: &Piece) {
    for row in 0..piece.size.1 {
      self.filled[piece.y + row] |= piece.masks[piece.x][row]
    }
    self.current_height = usize::max(self.current_height, piece.y + piece.size.1);
  }
}

/*
impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
     for row in (0..self.current_height).rev() {
       for col in 0..BOARD_WIDTH {
         write!(f, "{}", if (self.filled[row] >> col) & 1 == 1 {'#'} else {'.'});
       }
       write!(f, "\n");
     }
    write!(f, "\n")
  }
}
*/

struct WindIter<'a> {
  data: &'a[Wind],
  posn: usize,
}

impl<'a> WindIter<'a> {
  fn new(data: &'a[Wind]) -> Self {
    WindIter{data, posn: 0}
  }

  fn next(&mut self) -> Wind {
    let old = self.posn;
    self.posn = (self.posn + 1) % self.data.len();
    self.data[old]
  }

  fn get_posn(&self) -> usize {
    self.posn
  }
}

#[derive(Debug)]
struct CycleInformation {
  kind: usize,
  wind: usize,
  start: usize,
  height: usize,
}

#[derive(Debug)]
struct TailInformation {
  remaining_pieces: usize,
  extra_rows: usize,
}

pub fn drop_rocks(input: &InputType, count: usize) -> OutputType {
  let mut seen: HashSet<(usize,usize)> = HashSet::new();
  let mut cycle: Option<CycleInformation> = None;
  let mut tail: Option<TailInformation> = None;
  let mut board = Board::new(500_000);
  let mut wind_itr = WindIter::new(input);
  let mut pieces: Vec<Piece> = [PieceKind::Bar, PieceKind::Plus, PieceKind::L,
    PieceKind::I, PieceKind::Square].iter().map(|k| Piece::new(*k)).collect();
  let num_pieces = pieces.len();
  for piece_count in 0..count {
    // Check for cycles in the piece kind and wind
    let kind_and_wind = (piece_count % num_pieces, wind_itr.get_posn());
    // Are we computing the tail?
    if let Some(info) = &mut tail {
      // just count down the remaining rows
      if info.remaining_pieces == 0 {
        break;
      } else {
        info.remaining_pieces -= 1;
      }
    } else if let Some(info) = &cycle {
      // Check to see if we have found the second iteration of the cycle. If so,
      // compute the length of the cycle and how high our virtual stack needs to be.
      if (info.kind, info.wind) == kind_and_wind {
        let cycle_time = piece_count - info.start;
        let cycle_height = board.current_height - info.height;
        let remaining = count - piece_count;
        // Set the tail information so that we handle the last partial cycle
        tail = Some(TailInformation{remaining_pieces: (remaining - 1) % cycle_time,
          extra_rows: (remaining / cycle_time) * cycle_height});
        if remaining % cycle_time == 0 {
          break
        }
      }
    } else {
      // Look for a repeat of kind and wind to identify the cycle.
      // The first cycle is usually different, but the second and later cycles are
      // identical. After the first, we set the cycle information and use that to
      // identify the second iteration.
      if seen.contains(&kind_and_wind) {
        seen.clear();
        cycle = Some(CycleInformation{kind: kind_and_wind.0, wind: kind_and_wind.1,
          start: piece_count, height: board.current_height});
      } else {
        seen.insert(kind_and_wind);
      }
    }
    let piece = &mut pieces[piece_count % num_pieces];
    // simulate the initial 3 level fall
    piece.reset(board.current_height);
    for _ in 0..=NEW_PIECE_HEIGHT {
      let wind = wind_itr.next();
      if let Some(new_x) = wind.blow(piece.x, BOARD_WIDTH + 1 - piece.size.0) {
        piece.x = new_x;
      }
    }
    // now check
    while board.can_fall(piece) {
      piece.y -= 1;
      board.blow(piece, wind_itr.next());
    }
    board.place_piece(piece);
  }
  if let Some(info) = tail {
    board.current_height += info.extra_rows;
  }
  board.current_height
}

pub fn part1(input: &InputType) -> OutputType {
  drop_rocks(input, 2_022)
}

pub fn part2(input: &InputType) -> OutputType {
  drop_rocks(input, 1_000_000_000_000)
}

#[cfg(test)]
mod tests {
  use crate::day17::{generator, part1, part2};

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(40, input.len());
    assert_eq!(3068, part1(&input));
  }

  #[test]
  fn test_part2() {
    assert_eq!(1514285714288, part2(&generator(INPUT)));
  }

  const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";
}
