#[derive(Debug, PartialEq)]
pub enum DirectionOffset {
  N = 8isize,
  S = -8isize, 
  W = -1isize, 
  E = 1isize, 
  NW = 7isize, 
  SE = -7isize, 
  NE = 9isize, 
  SW = -9isize 
}

pub struct Move {
    /// The starting board square index
    start: usize,
    /// The destination board square index
    destination: usize,
}
