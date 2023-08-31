mod helpers;

use rand::rngs::SmallRng;
use rand::SeedableRng;
use std::fs::File;
use std::io::Write;

use helpers::{magic_mask, questions_and_answers, random_bitboard, NUM_MOVES};

use crate::{
    bitboard::BitBoard,
    generators::RaysGenerator,
    primitives::{piece::SlidingDirection, Square},
};

use super::prelude::ArrayGenerator;

// This structure is for the "Magic Bitboard" generation
#[derive(Debug, Copy, Clone)]
pub struct Magic {
    magic_number: BitBoard,
    mask: BitBoard,
    offset: u32,
    rightshift: u8,
}

// These numbers allow you to hash a set of blocking pieces, and get an index in the MOVES
// array to return the valid moves, given a set of blocking pieces.
// This will be generated here, but then put into the magic_gen.rs as a const array.
static mut MAGIC_NUMBERS: [[Magic; Square::AMOUNT]; SlidingDirection::SIZE] = [[Magic {
    magic_number: BitBoard::EMPTY,
    mask: BitBoard::EMPTY,
    offset: 0,
    rightshift: 0,
}; 64];
    SlidingDirection::SIZE];

// How many squares can a blocking piece be on for the rook?
static mut GENERATED_NUM_MOVES: usize = 0;

// This is the valid move lookup table.  This will be generated here, then put into
// the magic_gen.rs as a const array.
static mut MOVES: [BitBoard; NUM_MOVES] = [BitBoard::EMPTY; NUM_MOVES];

// When a MOVES bitboard is updated, update this with the rays that the MOVES bitboard
// may have set.  This helps with compressing the MOVES array.
static mut MOVE_RAYS: [BitBoard; NUM_MOVES] = [BitBoard::EMPTY; NUM_MOVES];

// Find a perfect hashing function for the move generation for a particular square and piece type
// Store the resulting move array in MOVES[cur_offset...], and return the next offset
// to be used
fn generate_magic(square: Square, sliding_direction: SlidingDirection, cur_offset: usize) -> usize {
    let (questions, answers) = questions_and_answers(square, sliding_direction);
    assert_eq!(questions.len().count_ones(), 1);
    assert_eq!(questions.len(), answers.len());
    let mask = magic_mask(square, sliding_direction);

    assert_eq!(questions.iter().fold(BitBoard::EMPTY, |b, n| b | *n), mask);
    assert_eq!(
        answers.iter().fold(BitBoard::EMPTY, |b, n| b | *n),
        RaysGenerator::get_rays(square, sliding_direction)
    );
    let mut new_offset = cur_offset;

    for i in 0..cur_offset {
        let mut found = true;
        for j in 0..answers.len() {
            unsafe {
                if MOVE_RAYS[i + j] & RaysGenerator::get_rays(square, sliding_direction)
                    != BitBoard::EMPTY
                {
                    found = false;
                    break;
                }
            }
        }
        if found {
            new_offset = i;
            break;
        }
    }

    let mut new_magic = Magic {
        magic_number: BitBoard::EMPTY,
        mask,
        offset: new_offset as u32,
        rightshift: ((questions.len() as u64).leading_zeros() + 1) as u8,
    };

    let mut done = false;
    let mut rng = SmallRng::seed_from_u64(0xDEADBEEF12345678);

    while !done {
        let magic_bitboard = random_bitboard(&mut rng);

        if (mask * magic_bitboard).count_bits() < 6 {
            continue;
        }

        let mut new_answers = vec![BitBoard::EMPTY; questions.len()];
        done = true;
        for i in 0..questions.len() {
            let j = (magic_bitboard * questions[i]).to_size(new_magic.rightshift);
            if new_answers[j] == BitBoard::EMPTY || new_answers[j] == answers[i] {
                new_answers[j] = answers[i];
            } else {
                done = false;
                break;
            }
        }
        if done {
            new_magic.magic_number = magic_bitboard;
        }
    }

    unsafe {
        MAGIC_NUMBERS[sliding_direction.to_index()][square.to_index()] = new_magic;

        for i in 0..questions.len() {
            let j = (new_magic.magic_number * questions[i]).to_size(new_magic.rightshift);
            MOVES[(new_magic.offset as usize) + j] |= answers[i];
            MOVE_RAYS[(new_magic.offset as usize) + j] |=
                RaysGenerator::get_rays(square, sliding_direction);
        }
        if new_offset + questions.len() < cur_offset {
            new_offset = cur_offset;
        } else {
            new_offset += questions.len();
        }
        GENERATED_NUM_MOVES = new_offset;
    }
    new_offset
}

pub struct MagicGenerator;
impl ArrayGenerator<[Magic; Square::AMOUNT], 2 /* SlidingDirection::SIZE */> for MagicGenerator {
    const NAME: &'static str = "MAGIC_NUMBERS";

    // NOTE: not used
    fn generate_index_value(_index: usize) -> [Magic; Square::AMOUNT] {
        unreachable!()
    }

    fn generate_array() -> [[Magic; Square::AMOUNT]; SlidingDirection::SIZE] {
        let mut cur_offset = 0;
        for direction in SlidingDirection::ALL.iter() {
            for square in Square::ALL.iter() {
                cur_offset = generate_magic(*square, *direction, cur_offset);
            }
        }

        unsafe {
            return MAGIC_NUMBERS;
        }
    }

    // Write the MAGIC_NUMBERS and MOVES arrays to the specified file.
    fn write_generated_array(file: &mut File) -> std::io::Result<()> {
        Self::generate_array();

        write!(file, "#[derive(Copy, Clone)]\n")?;
        write!(file, "struct Magic {{\n")?;
        write!(file, "    magic_number: BitBoard,\n")?;
        write!(file, "    mask: BitBoard,\n")?;
        write!(file, "    offset: u32,\n")?;
        write!(file, "    rightshift: u8\n")?;
        write!(file, "}}\n\n")?;

        write!(file, "const MAGIC_NUMBERS: [[Magic; 64]; 2] = [[\n")?;
        for i in 0..2 {
            for j in 0..64 {
                unsafe {
                    write!(file, "    Magic {{ magic_number: BitBoard({}), mask: BitBoard({}), offset: {}, rightshift: {} }},\n",
                    MAGIC_NUMBERS[i][j].magic_number.0,
                    MAGIC_NUMBERS[i][j].mask.0,
                    MAGIC_NUMBERS[i][j].offset,
                    MAGIC_NUMBERS[i][j].rightshift)?;
                }
            }
            if i != 1 {
                write!(file, "], [\n")?;
            }
        }
        write!(file, "]];\n")?;

        unsafe {
            write!(file, "const MOVES: [BitBoard; {}] = [ ", GENERATED_NUM_MOVES)?;
            for i in 0..GENERATED_NUM_MOVES {
                write!(file, "BitBoard({}), ", MOVES[i].0)?;
            }
        }
        write!(file, "];\n")?;
        
        return Ok(());
    }
}
