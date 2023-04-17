use engine::{core::game::Chess, notations::FEN};
#[allow(unused_imports)]

use engine::core::{
    board::{ChessBoard, File, Square, ASCIIBoard},
    piece::{ChessPiece, KnightType, PieceType},
    team::Team,
};

fn main() -> () {
  let game = Chess::default();

  let game = FEN::gamestate_from_fen("rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2").unwrap();


    let origin = Square::E6;

    let mut ascii = ASCIIBoard::new();

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
    
    ascii.mark(Square::C4);

    ascii.print(&game.board);
}
