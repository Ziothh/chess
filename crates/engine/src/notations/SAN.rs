use crate::{
    core::{
        game::Chess,
        instructions::{CastlingMove, CheckKind, EndOfGameState, Instruction, Move},
        piece::ChessPieceVariant,
        position,
        team::Team,
    },
    utils::enums::StrEnum,
};

/// Parses a Standard Algebraic Notation (SAN) move.
/// @see https://www.chessprogramming.org/Algebraic_Chess_Notation#SAN
/// @see https://en.wikipedia.org/wiki/Algebraic_notation_(chess)
pub fn parse_instruction(san_move: &str, legal_moves: &Vec<&Move>) -> anyhow::Result<Instruction> {
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
            Ok(Instruction::Castling(CastlingMove::KingSide))
        }
        "O-O-O" => {
            // Queenside castling
            Ok(Instruction::Castling(CastlingMove::QueenSide))
        }
        str => {
            let mut string = str.to_string();

            let bytes = string.as_bytes();

            // ! This should be implemented
            bytes.iter().enumerate().for_each(|(i, char_byte)| {
                // Currently not supporing annotations like blunders
                if char_byte == b'?' || char_byte == b'!' {
                    string.remove(i);
                }
            });


            // Example full move: c7xd8a=Q+
            // Pawn moves from c7 and takes on d8, then promotes to a Queen, checking the King
            // The origin (c7) is left (fully/partially: rank or file) out if the there are no other pieces that could go to the destination square

            let piece = {
                let first_char = bytes.first().unwrap();

                if first_char.is_ascii_uppercase() {
                    // Starts with piece type
                    string.remove(0);
                    ChessPieceVariant::from(first_char)
                } else {
                    // Starts with a positional description
                    ChessPieceVariant::Pawn
                }
            };

            let checks = {
                let check_variant = match bytes.last().unwrap() {
                    b'+' => Some(CheckKind::Check),
                    b'#' => Some(CheckKind::Mate),
                };

                if check_variant.is_some() {
                    string.pop();
                }

                check_variant
            };


            // String should have a minimal length of 2 right now

            let promotion = {
              if bytes.get(bytes.len() - 1) != b'=' { None }


              let promoting_piece = Some(ChessPieceVariant::from_str(string.pop().unwrap()).unwrap());

              string.pop();

              promoting_piece
            };

            let takes = {
                let res = bytes.binary_search(&b'x');

                if let Ok(index) = res {
                    string.remove(index);

                    true
                }
                false
            };

            // E.g. ..e7
            let destination = {
                // NOTE: .pop() order is important here
                let rank = string.pop().unwrap().to_digit(10).unwrap();
                let file = string.pop().unwrap();

                position::coords_to_index(file, rank)
            };

            let origin = {
              let resting_amount = string.len();

              // Full origin description
              if resting_amount == 2 {
                let rank = string.pop().unwrap();
                let file = string.pop().unwrap();

                position::coords_to_index(
                  position::file_char_to_number(&file), 
                  position::rank_char_to_number(&rank).unwrap()
                )
              }

              // Partial description: file or rank
              let identifier_char = string.pop();

              if let Some(identifier) = identifier_char {
                // Found one identifier
                if identifier.is_numeric {
                  // Rank
                  0
                } else {
                  // File
                  0
                }
              } else {
                // No identiefier
                0
              }
            };

            return Ok(Instruction::Move(Move {
                piece,
                origin,
                destination,
                takes,
                checks,
                promotion,
            }));
        }
    };
}

// Implementing str enums
impl StrEnum for ChessPieceVariant {
    fn to_str(&self) -> &str {
        use ChessPieceVariant::*;
        match self {
            Pawn => "",
            Bishop => "B",
            King => "K",
            Knight => "N",
            Queen => "Q",
            Rook => "R",
        }
    }
    fn from_str(value: &str) -> anyhow::Result<Self> {
        use ChessPieceVariant::*;
        match value {
            "" => Ok(Pawn),
            "B" => Ok(Bishop),
            "K" => Ok(King),
            "N" => Ok(Knight),
            "Q" => Ok(Queen),
            "R" => Ok(Rook),
            _ => Err(format!("invalid piece: {}", value)),
        }
    }
}

impl StrEnum for CastlingMove {
    fn to_str(&self) -> &str {
        match self {
            CastlingMove::Kingside => "O-O",
            CastlingMove::Queenside => "O-O-O",
        }
    }

    fn from_str(value: &str) -> anyhow::Result<CastlingMove> {
        match value {
            "O-O" => Ok(CastlingMove::Kingside),
            "O-O-O" => Ok(CastlingMove::Queenside),
            _ => Err(format!("invalid castling move: {}", value)),
        }
    }
}
