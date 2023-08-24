use engine::bitboard::attack_tables::pawn;
use engine::core::Rank;
#[allow(unused_imports)]
use engine::{core::game::Chess, notations::FEN};

#[allow(unused_imports)]
use engine::{
    bitboard::BitBoard,
    core::{
        board::{ASCIIBoard, ChessBoard, File, Square},
        team::Team,
    },
};

fn main() -> () {
    let sq = Square::C6;
    let team = Team::White;

    debug_square(sq);

    let mut bb = BitBoard::from(sq);

    bb.set_square(sq.uforward(team));
    bb.set_square(sq.uforward(team).uleft());
    bb.set_square(sq.uforward(team).uright());
    // bb.set_square(sq.ubackward(team));
    // bb.set_square(sq.ubackward(team).uleft());
    // bb.set_square(sq.ubackward(team).uright());

    println!("{sq}:\n{bb}");


    let mut bb = BitBoard::from(sq);

    bb.set_maybe_square(sq.forward(team));
    bb.set_maybe_square(sq.forward(team).and_then(|sq| sq.left()));
    bb.set_maybe_square(sq.forward(team).and_then(|sq| sq.right()));
    // bb.set_maybe_square(sq.backward(team));
    // bb.set_maybe_square(sq.backward(team).and_then(|sq| sq.left()));
    // bb.set_maybe_square(sq.backward(team).and_then(|sq| sq.right()));

    println!("{sq}:\n{bb}");

    let attack = pawn::mask_attacks(sq, team);
    println!("Attack:\n{attack}");


    // let attack_table = engine::bitboard::attack_tables::prelude::generate_attack_map(
    //     engine::bitboard::attack_tables::rook::mask_attacks,
    // );
    //
    // for rank in Rank::ALL.iter() {
    //     for file in File::ALL.iter() {
    //         let square = Square::make_square(*file, *rank);
    //
    //         let bb = attack_table.get(square.to_index()).unwrap();
    //
    //         // print!(
    //         //     "{}, ",
    //         //     engine::bitboard::attack_tables::generate_attack_map(
    //         //         engine::bitboard::attack_tables::rook::mask_attacks
    //         //     )
    //         //     .get(square.to_int() as usize)
    //         //     .unwrap()
    //         //     .count_bits()
    //         // )
    //
    //         print!("{}", bb);
    //
    //         println!("");
    //     }

    // println!("")
    // }

    // let bb = BitBoard::from_int(578712835584952320u64);

    // let ls1b = bb.ls1b_square().unwrap();
    //
    //
    //
    // println!(
    //     "{}",
    //     BitBoard::new([ls1b])
    //     // BitBoard::from_int(bb.to_int())
    //     // (bb.clone() & !bb.clone()).to_string()
    //     // BitBoard::new([
    //     //     // Squares with pieces
    //     //     Square::D7,
    //     //     Square::D2,
    //     //     Square::B4,
    //     //     Square::G4,
    //     // ])
    //     // .to_string()
    // );
    //
    // return;
    //
    // let mut bb = BitBoard::from(Square::E1);
    // println!("{bb}\n");
    //
    // bb.set_square(Square::A1);
    // println!("{bb}\n");
    //
    // bb.unset_square(Square::A1);
    // println!("{bb}\n");
    //
    // bb.unset_square(Square::A1);
    // println!("{bb}\n");
}

fn debug_square(square: Square) {
    println!(
        "Square({square}): {{ file: {}, rank: {}, index: {} }}",
        square.get_file(),
        square.get_rank(),
        square.to_index(),
    );
}

#[allow(dead_code)]
fn sim_game() -> () {
    // let mut game = Chess::default();
    let mut ascii = ASCIIBoard::new();

    let mut game =
        FEN::gamestate_from_fen("rnbqkb1r/ppp1pppp/5n2/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 2 3")
            .unwrap();

    game.make_move(Square::B2, Square::B3).unwrap();

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
