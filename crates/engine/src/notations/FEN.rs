/// The starting FEN string.
pub const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

use crate::{
    boards::ChessBoard,
    game::Chess,
    primitives::{
        board::{File, Rank, Square},
        piece::ChessPiece,
        team::Team,
    },
};

pub fn board_from_fen(board_str: &str) -> ChessBoard {
    let mut board = ChessBoard::empty();

    let mut offset;
    for (rank, row) in board_str.split("/").enumerate() {
        // Keeps track of the amount of empty squares of the current, indicated by a number in the FEN string
        offset = 0;

        for (file, char) in row.chars().enumerate() {
            if char.is_numeric() {
                offset += char.to_digit(10).unwrap() as usize - 1;
                continue;
            }

            board.set(
                Square::make_square(
                    File::from_index(file + offset),
                    Rank::from_index(Rank::SIZE - 1 - rank),
                ),
                ChessPiece::try_from(char).unwrap(),
            );
        }
    }

    return board;
}

/// This function parses a FEN string and returns the chess game state represented by it.
///
/// FEN is a notation standard to represent the game state of a chess game.
///
/// @TODO: Add better validaton
/// @TODO: Finish parsing every bit of info about it.
///
/// @see [Specs](https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation)
pub fn gamestate_from_fen(fen_string: &str) -> anyhow::Result<Chess> {
    let parts = Vec::from_iter(fen_string.split(" "));

    // Piece placement data: Each rank is described, starting with rank 8 and ending with rank 1, with a "/" between each one; within each rank, the contents of the squares are described in order from the a-file to the h-file. Each piece is identified by a single letter taken from the standard English names in algebraic notation (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K"). White pieces are designated using uppercase letters ("PNBRQK"), while black pieces use lowercase letters ("pnbrqk"). A set of one or more consecutive empty squares within a rank is denoted by a digit from "1" to "8", corresponding to the number of squares.
    let board_str = parts[0];
    //  "w" means that White is to move; "b" means that Black is to move.
    let team_to_move = match parts[1] {
        "w" => Team::White,
        "b" => Team::Black,
        token => unreachable!(r#"Token "{token}" is not a valid team identifier"#),
    };

    // Castling availability: If neither side has the ability to castle, this field uses the character "-". Otherwise, this field contains one or more letters: "K" if White can castle kingside, "Q" if White can castle queenside, "k" if Black can castle kingside, and "q" if Black can castle queenside. A situation that temporarily prevents castling does not prevent the use of this notation.
    // TODO
    let _castling_availability = parts[2];

    // En passant target square: This is a square over which a pawn has just passed while moving two squares; it is given in algebraic notation. If there is no en passant target square, this field uses the character "-". This is recorded regardless of whether there is a pawn in position to capture en passant.[6] An updated version of the spec has since made it so the target square is only recorded if a legal en passant move is possible but the old version of the standard is the one most commonly used.
    // TODO
    let _en_passant = parts[3];

    let board = board_from_fen(board_str);

    Ok(Chess {
        board: todo!(),
        team_to_move,
        halfmove_clock: parts[4].parse()?,
        fullmove_clock: parts[5].parse()?,
        en_passant: todo!(),
    })
}
