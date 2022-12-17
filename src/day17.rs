use std::fmt::{Display, Formatter};

type InputType = Vec<Wind>;
type OutputType = usize;

#[derive(Clone,Debug,Eq,PartialEq)]
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
    .map(| c | Wind::parse(c)).collect()
}

#[derive(Clone,Debug,Eq,PartialEq)]
enum PieceKind {
  Bar,
  Plus,
  El,
  I,
  Square,
}

impl PieceKind {
  /// Return the bitmask in the low bits
  fn shape(&self) -> &[u8] {
    match self {
      PieceKind::Bar => &[0xf],
      PieceKind::Plus => &[0x2, 0x7, 0x2],
      PieceKind::El => &[0x7, 0x4, 0x4],
      PieceKind::I => &[0x1; 4],
      PieceKind::Square => &[0x3; 2],
    }
  }

  fn size(&self) -> (usize,usize) {
    match self {
      PieceKind::Bar => (4, 1),
      PieceKind::Plus | PieceKind::El => (3, 3),
      PieceKind::I => (1, 4),
      PieceKind::Square => (2, 2),
    }
  }
}

const INITIAL_X: usize = 2;
const BOARD_WIDTH: usize = 7;

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
  fn blow(&self, piece: &mut Piece, wind: &Wind) {
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

const NUM_ROCKS: usize = 2022;

pub fn part1(input: &InputType) -> OutputType {
  let mut board = Board::new(NUM_ROCKS * 3);
  let mut wind_itr = input.iter().cycle();
  let mut pieces: Vec<Piece> = [PieceKind::Bar, PieceKind::Plus, PieceKind::El,
    PieceKind::I, PieceKind::Square].iter().map(|k| Piece::new(k.clone())).collect();
  let num_pieces = pieces.len();
  for piece_count in 0..NUM_ROCKS {
    let piece = &mut pieces[piece_count % num_pieces];
    // simulate the initial 3 level fall
    piece.reset(board.current_height);
    for _ in 0..=NEW_PIECE_HEIGHT {
      let wind = wind_itr.next().unwrap();
      if let Some(new_x) = wind.blow(piece.x, BOARD_WIDTH + 1 - piece.size.0) {
        piece.x = new_x;
      }
    }
    // now check
    while board.can_fall(piece) {
      piece.y -= 1;
      board.blow(piece, wind_itr.next().unwrap());
    }
    board.place_piece(piece);
  }
  board.current_height
}

pub fn part2(_: &InputType) -> OutputType {
  0
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
    //assert_eq!(93, part2(&generator(INPUT)));
  }

  const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>\n";
}
