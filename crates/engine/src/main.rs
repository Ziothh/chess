#[allow(unused_imports)]
use engine::{core::game::Chess, notations::FEN};

use engine::core::{
    board::{ASCIIBoard, ChessBoard, File, Square},
    team::Team,
};

fn main() -> () {
    let mut game = Chess::default();
    let mut ascii = ASCIIBoard::new();

    let mut game =
        FEN::gamestate_from_fen("rnbqkb1r/ppp1pppp/5n2/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3")
            .unwrap();

    game.make_move(Square::B2, Square::B3);

    ascii.print(&game.board);

    // let origin = Square::E6;


    game.generate_legal_moves()
        .iter()
        .filter(|m| m.origin == Square::D8)
        .for_each(|m| {
            println!("{:#?}", &m);
            ascii.mark(m.destination);
        });

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

    // ascii.mark(Square::C4);

    ascii.print(&game.board);
}
