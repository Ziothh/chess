use engine::core::{
    board::{ChessBoard, File, Square},
    piece::{ChessPiece, KnightType, PieceType},
    team::Team,
};

fn main() -> () {
    let knight_square = Square::E5;

    let x = KnightType::pseudo_legal_moves(knight_square, Team::Black);
    let mut board = ChessBoard::empty();

    board.set_piece(knight_square.to_index(), ChessPiece::try_from('N').unwrap());

    x.iter().for_each(|m| {
        board.set_piece(m.destination.to_index(), ChessPiece::try_from('n').unwrap());
    });

    board.print_ascii();
}
