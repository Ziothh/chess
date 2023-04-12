use std::{fmt::Display, hint::unreachable_unchecked};

use crate::{
    moves::Move,
    piece::{ChessPiece, ChessPieceColor, ChessPieceVariant},
};

pub const CHESSBOARD_WIDTH: usize = 8;
pub const CHESSBOARD_SIZE: usize = CHESSBOARD_WIDTH * CHESSBOARD_WIDTH;
const EMPTY_CELL: ChessBoardCell = None;
const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
/// The amount needed to substract from a letter where 'a' = 1, 'b' = 2, ...
const LOWERCASE_UTF8_OFFSET: u32 = 'a' as u32 - 1;

type ChessBoardCell = Option<ChessPiece>;
type ChessBoard = [ChessBoardCell; CHESSBOARD_SIZE];

#[derive(Debug)]
pub struct Chess {
    pub board: ChessBoard,

    pub current_color: ChessPieceColor,
}

impl Chess {
    /// TODO: Add better validaton
    /// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    pub fn from_fen(fen_string: &str) -> Self {
        let parts = Vec::from_iter(fen_string.split(" "));

        // Piece placement data: Each rank is described, starting with rank 8 and ending with rank 1, with a "/" between each one; within each rank, the contents of the squares are described in order from the a-file to the h-file. Each piece is identified by a single letter taken from the standard English names in algebraic notation (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K"). White pieces are designated using uppercase letters ("PNBRQK"), while black pieces use lowercase letters ("pnbrqk"). A set of one or more consecutive empty squares within a rank is denoted by a digit from "1" to "8", corresponding to the number of squares.
        let pieces = parts[0];
        //  "w" means that White is to move; "b" means that Black is to move.
        let current_color = match parts[1] {
            "w" => ChessPieceColor::White,
            "b" => ChessPieceColor::Black,
            _ => unreachable!(),
        };

        // Castling availability: If neither side has the ability to castle, this field uses the character "-". Otherwise, this field contains one or more letters: "K" if White can castle kingside, "Q" if White can castle queenside, "k" if Black can castle kingside, and "q" if Black can castle queenside. A situation that temporarily prevents castling does not prevent the use of this notation.
        let _castling_availability = parts[2];

        // En passant target square: This is a square over which a pawn has just passed while moving two squares; it is given in algebraic notation. If there is no en passant target square, this field uses the character "-". This is recorded regardless of whether there is a pawn in position to capture en passant.[6] An updated version of the spec has since made it so the target square is only recorded if a legal en passant move is possible but the old version of the standard is the one most commonly used.
        let _en_passant = parts[3];

        // Halfmove clock: The number of halfmoves since the last capture or pawn advance, used for the fifty-move rule.
        let _halfmove_clock = parts[4];
        // Fullmove number: The number of the full moves. It starts at 1 and is incremented after Black's move.
        let _fullmove_clock = parts[5];

        let mut board: ChessBoard = [EMPTY_CELL; CHESSBOARD_SIZE];

        let mut current_index: usize = 0;
        for row in pieces.split("/") {
            for char in row.chars() {
                if char.is_numeric() {
                    current_index += char.to_digit(10).unwrap() as usize;
                    continue;
                }

                board[current_index] = Some(ChessPiece::try_from(char).unwrap());

                current_index += 1;
            }
        }

        Self {
            board,
            current_color,
        }
    }

    pub fn generate_legal_moves(&self) -> Vec<Move> {
        self.board.iter().enumerate().map(|(i, piece)| {});

        todo!()
    }

    /// Parses a Standard Algebraic Notation (SAN) move.
    /// @see https://www.chessprogramming.org/Algebraic_Chess_Notation#SAN
    pub fn parse_move(&self, san_move: &str) -> anyhow::Result<Move> {
        // Generate all legal moves
        let moves = self.generate_legal_moves();

        // Parse the move

        return match san_move {
            "1-0" => {
                // White wins
            }
            "0-1" => {
                // Black wins
            }
            "O-O" => {
                // Kingside castling
            }
            "O-O-O" => {
                // Queenside castling
            }
            str => {
                let length = str.len();

                let mut takes = false;
                let mut destination: u32 = 0;
                let mut origin: Option<u32> = None;
                let mut variant = ChessPieceVariant::Pawn;

                let mut num = 0;
                // Example: Nxe7 but reversed
                for (i, x) in str.chars().rev().enumerate() {
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
                            variant = ChessPieceVariant::try_from(x).unwrap();
                        }
                    }

                    if i < 2 {
                        destination += num;
                    } else {
                        origin = Some(origin.unwrap_or(0) + num)
                    }
                }

                Move
            }
        };
    }

    /**
      Makes a move, defined in the PGN (Portable Page Notation) standard.
      Example: e4
      https://en.wikipedia.org/wiki/Portable_Game_Notation
    */
    pub fn make_move(&mut self, san_move: &str) -> anyhow::Result<&mut Self> {
        let generated_move = self.parse_move(san_move)?;

        // Apply the move

        todo!()
    }
}

impl Default for Chess {
    fn default() -> Self {
        Chess::from_fen(STARTING_FEN)
    }
}

impl Display for Chess {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.board
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let mut string_repr = if let Some(piece) = cell {
                        piece.to_string()
                    } else {
                        ".".to_string()
                    };

                    if i % 8 == 0 && i != 0 {
                        string_repr.insert_str(0, "\n");
                    }

                    return string_repr;
                })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
