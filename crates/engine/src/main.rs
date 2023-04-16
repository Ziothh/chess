use engine::core::{
    board::{ChessBoard, File, Square},
    piece::{ChessPiece, KnightType, PieceType},
    team::Team,
};

fn main() -> () {
    let origin = Square::E5;

    // let x = KnightType::pseudo_legal_moves(knight_square, Team::Black);
    let mut board = ChessBoard::empty();

    let piece = ChessPiece::try_from('r').unwrap();

    board.set_piece(origin.to_index(), piece);

    let x = board
        .get(origin.to_index())
        .as_ref()
        .unwrap()
        .pseudo_legal_moves(origin);

    x.iter().for_each(|m| {
        board.set_piece(m.destination.to_index(), ChessPiece::try_from('n').unwrap());
    });

    board.print_ascii();
}
