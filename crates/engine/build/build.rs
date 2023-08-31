#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
#![feature(const_trait_impl)]

use std::{fs::File, io::Write, path::Path};

#[path = "../src/bitboard/mod.rs"]
mod bitboard;
#[path = "../src/primitives/mod.rs"]
mod primitives;

mod generators;
use generators::prelude::{ArrayGenerator, ValueGenerator};

fn main() {
    // let now = std::time::SystemTime::now();
    // let time = {
    //     let tmp = format!("{:?}.txt", now).replace("SystemTime { tv_sec: ", "");
    //     tmp.split_at(tmp.find(',').unwrap()).0.to_owned()
    // };
    //
    // write!(
    //     File::create(
    //         Path::new("/home/zioth/projects/apps/chess/crates/engine/data/built_at")
    //             .join(format!("{:?}.txt", time).replace("\"", ""))
    //     )
    //     .unwrap(),
    //     "{:?}",
    //     now,
    // )
    // .unwrap();

    // TODO: move this to OUT_DIR
    // let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "/home/zioth/projects/apps/chess/crates/engine/data";
    let magic_path = Path::new(&out_dir).join("magic_gen.rs");
    // if magic_path.metadata().is_ok_and(|f| f.is_file()) { 
    //     return; // Don't regenerate everything if the file already exists
    // }
    let mut file = File::create(&magic_path).unwrap();


    generators::LinesGenerator::write_generated_array(&mut file).unwrap();
    generators::BetweenGenerator::write_generated_array(&mut file).unwrap();
    generators::RaysGenerator::write_generated_array(&mut file).unwrap();

    generators::KnightAttacksGenerator::write_generated_array(&mut file).unwrap();
    
    generators::KingAttacksGenerator::write_generated_array(&mut file).unwrap();
    generators::KingsideCastleSquaresGenerator::write_generated_array(&mut file).unwrap();
    generators::QueensideCastleSquaresGenerator::write_generated_array(&mut file).unwrap();
    generators::CastleMovesGenerator::write_generated_value(&mut file).unwrap();

    generators::PawnMovesGenerator::write_generated_array(&mut file).unwrap();
    generators::PawnAttacksGenerator::write_generated_array(&mut file).unwrap();
    generators::PawnDoubleMoveOriginsGenerator::write_generated_value(&mut file).unwrap();
    generators::PawnDoubleMoveDestinationsGenerator::write_generated_value(&mut file).unwrap();

    generators::MagicGenerator::write_generated_array(&mut file).unwrap();

    // println!("BUILD SCRIPT RUNNING");
}
