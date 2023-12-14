use crate::primitives::{
    board::Square,
    moves::Move,
    piece::{Piece, PieceType},
    team::Team,
};

// const DIRECTIONS_LONG: [isize; 2] = [2, -2];
// const DIRECTIONS_SHORT: [isize; 2] = [1, -1];

pub struct KnightType;

impl PieceType for KnightType {
    const PIECE_VARIANT: Piece = Piece::Knight;

    fn pseudo_legal_moves(position: Square, _team: Team) -> Vec<Move> {
        // let moves: Vec<_> = Vec::new();

        // let mut long_y_moves: Vec<_> = DIRECTIONS_LONG
        //     .iter()
        //     .map(|y| DIRECTIONS_SHORT.iter().map(|x| (x, y)).collect::<Vec<_>>())
        //     .collect();
        // let mut long_x_moves: Vec<_> = DIRECTIONS_LONG
        //     .iter()
        //     .map(|x| DIRECTIONS_SHORT.iter().map(|y| (x, y)).collect::<Vec<_>>())
        //     .collect();

        // long_x_moves.append(&mut long_y_moves);

        // knight_square.translate(x, i)

        // let dir_moves: Vec<_> = long_x_moves.into_iter().flatten().collect();

        // println!("{:#?}", dir_moves);

        return DIRECTIONS
            .iter()
            .filter_map(|(x, y)| {
                if let Some(destination) = position.translate(*x, *y) {
                    Some(Move::new(Self::PIECE_VARIANT, position, destination))
                } else {
                    None
                }
            })
            .collect();
    }
}
