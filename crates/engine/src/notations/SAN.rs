use crate::{
    primitives::{
        board::{File, Rank, Square},
        instructions::{CastlingMove, CheckKind, EndOfGameState, Instruction},
        moves::Move,
        piece::ChessPieceVariant,
        team::Team,
    },
    utils::enums::StrEnum,
};

/// Parses a Standard Algebraic Notation (SAN) string into a `Move`.
///
/// SAN is used to represent chess moves with as litle characters as needed. 
/// It relies heavilly on the context of the game state.
/// 
/// @TODO: take the legal moves into a count.
///
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
            let mut chars: Vec<char> = str
                .chars()
                // ! This should be implemented
                // Currently not supporing annotations like blunders
                .filter(|char| *char == '?' || *char == '!')
                .collect();

            // Example full move: c7xd8a=Q+
            // Pawn moves from c7 and takes on d8, then promotes to a Queen, checking the King
            // The origin (c7) is left (fully/partially: rank or file) out if the there are no other pieces that could go to the destination square

            let piece = {
                let first_char = chars.first().unwrap().clone();

                if first_char.is_ascii_uppercase() {
                    // Starts with piece type
                    chars.remove(0);
                    ChessPieceVariant::try_from(first_char).unwrap()
                } else {
                    // Starts with a positional description
                    ChessPieceVariant::Pawn
                }
            };

            let checks = {
                let check_variant = match *chars.last().unwrap() {
                    // Todo impl StrEnum for CheckKind
                    '+' => Some(CheckKind::Check),
                    '#' => Some(CheckKind::Mate),
                    _ => None,
                };

                if check_variant.is_some() {
                    chars.pop();
                }

                check_variant
            };

            // String should have a minimal length of 2 right now

            let promotion: Option<ChessPieceVariant> = {
                if (*chars.get(chars.len() - 1).unwrap()) != '=' {
                    None
                } else {
                    let promoting_piece =
                        Some(ChessPieceVariant::try_from(chars.pop().unwrap()).unwrap());

                    chars.pop();

                    promoting_piece
                }
            };

            let takes = {
                let res = chars.binary_search(&'x');

                if let Ok(index) = res {
                    chars.remove(index);
                    true
                } else {
                    false
                }
            };

            // E.g. ..e7
            let destination = {
                // NOTE: .pop() order is important here
                let rank = chars.pop().unwrap();
                let file = chars.pop().unwrap();

                Square::make_square(File::try_from(file).unwrap(), Rank::try_from(rank).unwrap())
            };

            let origin: Square = {
                let resting_amount = chars.len();

                // Full origin description
                if resting_amount == 2 {
                    let rank = chars.pop().unwrap();
                    let file = chars.pop().unwrap();

                    Square::make_square(
                        File::try_from(file).unwrap(),
                        Rank::try_from(rank).unwrap(),
                    )
                } else {
                    // Partial description: file or rank
                    let identifier_char = chars.pop();

                    if let Some(identifier) = identifier_char {
                        // Found one identifier
                        if identifier.is_numeric() {
                            // Rank
                            todo!()
                        } else {
                            // File
                            todo!()
                        }
                    } else {
                        // No identiefier
                        todo!()
                    }
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
    type Error = String;
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
    fn from_str(value: &str) -> Result<Self, Self::Error> {
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
    type Error = String;
    fn to_str(&self) -> &str {
        match self {
            CastlingMove::KingSide => "O-O",
            CastlingMove::QueenSide => "O-O-O",
        }
    }

    fn from_str(value: &str) -> Result<Self, Self::Error> {
        match value {
            "O-O" => Ok(CastlingMove::KingSide),
            "O-O-O" => Ok(CastlingMove::QueenSide),
            _ => Err(format!("invalid castling move: {}", value)),
        }
    }
}
