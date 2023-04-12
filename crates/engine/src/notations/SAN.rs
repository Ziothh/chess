use crate::core::{
    instructions::{CaslingMove, EndOfGameState, Instruction, Move},
    piece::ChessPieceVariant,
    team::Team, game::Chess,
};

/// The amount needed to substract from a letter where 'a' = 1, 'b' = 2, ...
const LOWERCASE_UTF8_OFFSET: u32 = 'a' as u32 - 1;

/// Parses a Standard Algebraic Notation (SAN) move.
/// @see https://www.chessprogramming.org/Algebraic_Chess_Notation#SAN
pub fn parse_instruction(chess_game: &Chess, san_move: &str) -> anyhow::Result<Instruction> {
    // Generate all legal moves
    let moves = chess_game.generate_legal_moves();

    // Parse the move
    return match san_move {
        "1-0" => {
            // White wins
            Ok(Instruction::EndOfGame(EndOfGameState::Win(Team::White)))
        }
        "0-1" => {
            // Black wins
            Ok(Instruction::EndOfGame(EndOfGameState::Win(Team::Black)))
        }
        "1/2-1/2" => {
            // Draw
            Ok(Instruction::EndOfGame(EndOfGameState::Draw))
        }
        "O-O" => {
            // Kingside castling
            Ok(Instruction::Castling(CaslingMove::KingSide))
        }
        "O-O-O" => {
            // Queenside castling
            Ok(Instruction::Castling(CaslingMove::QueenSide))
        }
        str => {
            let length = str.len();

            let mut takes = false;
            let mut destination: u32 = 0;
            let mut origin: Option<u32> = None;
            let mut piece = ChessPieceVariant::Pawn;

            let mut num = 0;
            // Example: Nxe7 but reversed
            for (i, x) in str.chars().rev().enumerate() {
                // TODO: handle this + character Rh7+
                if x.is_numeric() {
                    // Rank indication
                    num = char::to_digit(x, 10).unwrap();
                } else {
                    if x == 'x' {
                        // It takes
                        takes = true;
                        continue;
                    }

                    if x.is_lowercase() {
                        // It is a file indication
                        num = x as u32 - LOWERCASE_UTF8_OFFSET;
                    } else {
                        // It is a piecetype
                        piece = ChessPieceVariant::try_from(x).unwrap();
                    }
                }

                if i < 2 {
                    destination += num;
                } else {
                    origin = Some(origin.unwrap_or(0) + num)
                }
            }

            let origin = origin.unwrap_or({
              todo!()
            });

            Ok(Instruction::Move(Move {
                origin,
                destination,
                takes,
                piece,
            }))
        }
    };
}

// use crate::game::CHESSBOARD_WIDTH;
//
//
// /** Generates a chessboard position string from the position index (0..64).
//  * Example: index `56` = `"a7"` = top left corner */
// pub fn position_to_string(position: usize) -> String {
//     let x = position % CHESSBOARD_WIDTH;
//     let y = position / CHESSBOARD_WIDTH;
//
//     return format!(
//         "{}{y}",
//         char::from_digit((10 + x) as u32, 36).unwrap().to_string()
//     );
// }
//
// pub fn position_from_string(position: &str) -> usize {
//     let mut chars = position.chars();
//
//     let x = chars.next().unwrap() as usize - 97;
//     let y = chars.next().unwrap().to_digit(10).unwrap() as usize;
//
//     x + y * CHESSBOARD_WIDTH
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn positions() {
//         (0..64).for_each(|index| {
//             let str_repr = position_to_string(index);
//             let num_repr = position_from_string(&str_repr);
//
//             assert_eq!(index, num_repr);
//         });
//     }
// }
