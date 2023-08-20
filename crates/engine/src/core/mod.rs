pub mod game;

pub mod board;
pub use board::*;

pub mod piece;
pub mod instructions;

pub mod team;
pub use team::*;

pub mod moves;

// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   #[test]
//   fn it_works() {
//     let result = add(2, 2);
//     assert_eq!(result, 4);
//   }
// }
