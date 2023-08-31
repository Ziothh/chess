/*
For some reason I've yet to fully grasp, the A and H files are not included in 
the attack map BitBoards.

This is probably because most engines were writtin in C and wanted to prevent wrapping.
The current Square implementation has methods that prevent the bit wrapping 
around to the other square. 

I will implement it as the wiki states, but maybe I can optimise this later on.
*/

mod prelude;

pub mod bishop;
mod king;
mod knight;
pub mod pawn;
pub mod rook;
