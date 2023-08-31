#![feature(generic_const_exprs)]
#![feature(associated_type_defaults)]

use std::{fs::File, io::Write, path::Path};

#[path = "../src/bitboard/mod.rs"]
mod bitboard;
#[path = "../src/primitives/mod.rs"]
mod primitives;

mod core;
use crate::core::ArrayGenerator;

mod generators;

fn main() {
    let now = std::time::SystemTime::now();
    let time = {
        let mut tmp = format!("{:?}.txt", now).replace("SystemTime { tv_sec: ", "");
        tmp.split_at(tmp.find(',').unwrap()).0.to_owned()
    };

    write!(
        File::create(
            Path::new("/home/zioth/projects/apps/chess/crates/engine/data/built_at")
                .join(format!("{:?}.txt", time).replace("\"", ""))
        )
        .unwrap(),
        "{:?}",
        now,
    )
    .unwrap();

    // let out_dir = env::var("OUT_DIR").unwrap();
    let out_dir = "/home/zioth/projects/apps/chess/crates/engine/data";
    let magic_path = Path::new(&out_dir).join("magic_gen.rs");
    let mut file = File::create(&magic_path).unwrap();

    generators::LinesGenerator::write_generated_array(&mut file);

    // println!("BUILD SCRIPT RUNNING");
}