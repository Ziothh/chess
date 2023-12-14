#![feature(const_trait_impl)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(inline_const)]
pub mod primitives;
pub use primitives::*;
pub mod boards;
pub mod bitboard;
pub mod notations;
pub mod utils;

mod generated;
pub use generated::*;

pub mod game;
pub use game::*;
