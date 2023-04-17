use engine::core::game::Chess;
#[allow(unused_imports)]

use engine::core::{
    board::{ChessBoard, File, Square},
    piece::{ChessPiece, KnightType, PieceType},
    team::Team,
};

fn main() -> () {
  let game = Chess::default();


    let origin = Square::E6;

    // let mut board = ChessBoard::empty();

    // let piece = ChessPiece::try_from('P').unwrap();
    //
    // board.set_piece(origin.to_index(), piece);
    //
    // let x = board
    //     .get(origin.to_index())
    //     .as_ref()
    //     .unwrap()
    //     .pseudo_legal_moves(origin);
    //
    // x.iter().for_each(|m| {
    //     board.set_piece(m.destination.to_index(), ChessPiece::try_from('n').unwrap());
    // });

    game.board.print_ascii();
}
