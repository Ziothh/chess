use engine::bitboard::magic::find_magic_number;
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
    fn init_magic_numbers() {
        use engine::bitboard::attack_tables;
        use engine::core::piece::SlidingDirection;


        Square::ALL.iter().for_each(|square| {
            println!("{square}");
            let attack_map = attack_tables::bishop::mask_attacks(*square);
            let relevant_bits = attack_map.count_bits();
            let magic_number = find_magic_number(*square, relevant_bits, SlidingDirection::Diagonal);

            println!("{:X}", magic_number.0);


        });
    }

    init_magic_numbers();


    // for rank in Rank::ALL.iter() {
    //     for file in File::ALL.iter() {
    //         let square = Square::make_square(*file, *rank);
    //
    //         let attacks = engine::bitboard::attack_tables::rook::mask_attacks(square);
    //         print!("{}, ", attacks.count_bits());
    //
    //     }
    //
    //     print!("\n")
    // }
    // println!("{}", gen_random_u32_number());
    // println!("{}", gen_random_u32_number() & 0xFFFF);
    // println!("{}", gen_random_u64_number());
    // for i in 0..3 {
    //     println!("{}", gen_random_u64_numbers());
    //
    // }
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
