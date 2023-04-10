use engine::{game::Chess, moves::DirectionOffset};

fn main() -> () {
  // let games = std::io::BufReader::new(std::fs::File::open("./data/games.txt").unwrap());

  println!("{:?}", DirectionOffset::N as isize)

  // let mut games = include_str!("../data/games.txt").split("\n");
  //
  // let moves = games.next().unwrap().split_whitespace();
  //
  // let mut board = engine::game::Chess::default();
  //
  // println!("Game start");
  // println!("{}", &board);
  //
  // moves.enumerate().for_each(|(i, m)| {
  //   println!("Move: {:#?}", m);
  //
  //   board.make_move(m);
  // });



  // let board = engine::board::ChessBoard::from_fen(&"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".replace("/",  ""));


  // println!("{:#?}", board.grid);
    

  // (0..64).for_each(|x| println!("{}", ChessBoard::position_to_string(x)));
  // println!("{}", ChessBoard::position_to_string(57));
  // println!("{}", ChessBoard::position_from_string("b7"));
}
