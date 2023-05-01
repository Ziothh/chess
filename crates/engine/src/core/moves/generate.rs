use std::{collections::HashMap, usize};

use crate::core::{
    board::{ChessBoard, Square, NUM_FILES},
    moves::Move,
    piece::{ChessPiece, ChessPieceVariant},
    team::Team,
};

pub type SquareTranslation = fn(origin: Square) -> Option<Square>;
const TRANSLATIONS: [SquareTranslation; 8] = [
    // Horizontal + vertical
    |origin| origin.up(),    // N
    |origin| origin.right(), // E
    |origin| origin.down(),  // S
    |origin| origin.left(),  // W
    // Diagonal
    |origin| origin.up().and_then(|sq| sq.right()), // NE
    |origin| origin.up().and_then(|sq| sq.left()),  // NW
    |origin| origin.down().and_then(|sq| sq.right()), // SE
    |origin| origin.down().and_then(|sq| sq.left()), // SW
];

const KNIGHT_DIRECTIONS: [SquareTranslation; 8] = [
    |origin: Square| origin.translate(2, 1),
    |origin: Square| origin.translate(2, -1),
    |origin: Square| origin.translate(-2, 1),
    |origin: Square| origin.translate(-2, -1),
    |origin: Square| origin.translate(1, 2),
    |origin: Square| origin.translate(-1, 2),
    |origin: Square| origin.translate(1, -2),
    |origin: Square| origin.translate(-1, -2),
];

/// Todo calc attackers
pub fn generate_move_data(
    piece: &ChessPiece,
    square: Square,
    chessboard: &ChessBoard,
) -> (Vec<Vec<Square>>, Vec<Move>) {
    // let map = HashMap::new();

    let rays: Vec<Vec<Square>> = match piece.variant {
        ChessPieceVariant::Pawn => [
            square.forward(piece.team),
            // En passant
            if piece.team.get_nth_rank(2) == square.get_rank() {
                square
                    .forward(piece.team)
                    .and_then(|sq| sq.forward(piece.team))
            } else {
                None
            },
            // Attacking rays
            square
                .forward(piece.team)
                .and_then(|sq| sq.left())
                .and_then(|sq| chessboard.get(sq).as_ref().and(Some(sq))),
            square
                .forward(piece.team)
                .and_then(|sq| sq.right())
                .and_then(|sq| chessboard.get(sq).as_ref().and(Some(sq))),
        ]
        .iter()
        // Make direction for every entry
        .filter_map(|sq| {
            if let Some(s) = sq {
                Some(vec![*s])
            } else {
                None
            }
        })
        .collect(),
        _ => generate_default_rays(&piece, square),
    };
    // .map(|direction| !chessboard.get(sq).is_some_and(|p| p.team == piece.team))

    let moves: Vec<Move> = match piece.variant {
        ChessPieceVariant::Pawn => rays
            .iter()
            .map(|ray| {
                ray.iter()
                    .filter_map(|destination| {
                        let mut takes = false;
                        if let Some(p) = chessboard.get(*destination) {
                            if p.team == piece.team || destination.get_file() == square.get_file() {
                                return None;
                            } else {
                                takes = true;
                            }
                        }

                        return Some(Move {
                            origin: square,
                            destination: *destination,
                            piece: piece.variant,
                            takes,
                            checks: None,
                            // TODO
                            promotion: None,
                        });
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect(),
        _ => rays
            .iter()
            .map(|ray| {
                ray.iter()
                    .scan(false, |should_break, destination| {
                        let mut takes = false;

                        if *should_break {
                            return None;
                        }

                        if let Some(p) = chessboard.get(*destination) {
                            if p.team == piece.team {
                                return None;
                            } else {
                                // Take and stop generating moves
                                takes = true;
                                *should_break = !*should_break;
                            }
                        }

                        return Some(Move {
                            piece: piece.variant,
                            origin: square,
                            destination: *destination,
                            // TODO
                            checks: None,
                            promotion: None,
                            takes,
                        });
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect(),
    };

    return (rays, moves);
}

fn generate_default_rays(piece: &ChessPiece, origin: Square) -> Vec<Vec<Square>> {
    // let translations: [SquareTranslation] = ;
    match piece.variant {
        ChessPieceVariant::Queen | ChessPieceVariant::King => &TRANSLATIONS[..],
        ChessPieceVariant::Rook => &TRANSLATIONS[..4],
        ChessPieceVariant::Bishop => &TRANSLATIONS[4..],
        ChessPieceVariant::Knight => &KNIGHT_DIRECTIONS[..],
        ChessPieceVariant::Pawn => unreachable!("Pawn moves should be handled elsewhere"),
    }
    .iter()
    .map(|translation| {
        (0..if piece.variant.is_sliding() {
            NUM_FILES
        } else {
            1
        })
            .scan(origin, |current_dest, _| {
                // Check if the the square is inside of the board, breaks if not
                translation(*current_dest).and_then(|desitination| {
                    *current_dest = desitination;

                    Some(desitination)
                })
            })
            .collect()
    })
    .collect()

    // translations.iter
}

// fn generate_rays_from_translations(translations: &[SquareTranslation]) -> ! {
//     todo!()
// }
