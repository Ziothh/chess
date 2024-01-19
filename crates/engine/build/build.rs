#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]
#![feature(const_trait_impl)]

use std::{fs::File, io::Write, path::Path};

#[path = "../src/utils/mod.rs"]
mod utils;
#[path = "../src/bitboard/mod.rs"]
mod bitboard;
#[path = "../src/primitives/mod.rs"]
mod primitives;

mod generators;
use generators::prelude::{ArrayGenerator, ValueGenerator};

fn main() {
    println!("cargo:rerun-if-changed=build"); // Only rerun if build script has changed

    let magic_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data/magic_gen.rs");
    if magic_path.metadata().is_ok_and(|f| f.is_file()) { 
        return; // Don't regenerate everything if the file already exists
    }
    let mut file = File::create(&magic_path).unwrap();


    generators::LinesGenerator::write_generated_array(&mut file).unwrap();
    generators::BetweenGenerator::write_generated_array(&mut file).unwrap();
    generators::RaysGenerator::write_generated_array(&mut file).unwrap();

    generators::KnightAttacksGenerator::write_generated_array(&mut file).unwrap();
    
    generators::KingAttacksGenerator::write_generated_array(&mut file).unwrap();

    generators::PawnMovesGenerator::write_generated_array(&mut file).unwrap();
    generators::PawnAttacksGenerator::write_generated_array(&mut file).unwrap();
    generators::PawnDoubleMoveOriginsGenerator::write_generated_value(&mut file).unwrap();
    generators::PawnDoubleMoveDestinationsGenerator::write_generated_value(&mut file).unwrap();

    generators::MagicGenerator::write_generated_array(&mut file).unwrap();
}
