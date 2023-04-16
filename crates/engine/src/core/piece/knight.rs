use crate::core::{board::Square, team::Team, moves::Move, piece::{PieceType, ChessPieceVariant}};

const DIRECTIONS_LONG: [isize; 2] = [2, -2];
const DIRECTIONS_SHORT: [isize; 2] = [1, -1];

pub struct KnightType;

impl PieceType for KnightType {
    const PIECE_VARIANT: ChessPieceVariant = ChessPieceVariant::Knight;

    fn pseudo_legal_moves(position: Square, team: Team) -> Vec<Move> {
        let moves: Vec<_> = Vec::new();

        let mut long_y_moves: Vec<_> = DIRECTIONS_LONG.iter().map(|y| DIRECTIONS_SHORT.iter().map(|x| { (x, y) }).collect::<Vec<_>>()).collect();
        let mut long_x_moves: Vec<_> = DIRECTIONS_LONG.iter().map(|x| DIRECTIONS_SHORT.iter().map(|y| { (x, y) }).collect::<Vec<_>>()).collect();

        let dir_moves = long_x_moves.append(&mut long_y_moves);

        println!("{:#?}", dir_moves);

        return moves;
    }
}
