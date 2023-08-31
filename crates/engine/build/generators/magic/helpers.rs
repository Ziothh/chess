use rand::Rng;

use crate::{
    bitboard::BitBoard,
    generators::{RaysGenerator, prelude::{BISHOP_TRANSLATIONS, ROOK_TRANSLATIONS}},
    primitives::{
        piece::SlidingDirection,
        Square,
        File,
        Rank,
    },
};

// How many squares can a blocking piece be on for the rook?
const ROOK_BITS: usize = 12;
// How many squares can a blocking piece be on for a bishop?
const BISHOP_BITS: usize = 9;
// How many different sets of moves for both rooks and bishops are there?
pub const NUM_MOVES: usize = 64 * (1<<ROOK_BITS) /* Rook Moves */ +
                         64 * (1<<BISHOP_BITS) /* Bishop Moves */;

// Generate a random bitboard with a small number of bits.
pub fn random_bitboard<R: Rng>(rng: &mut R) -> BitBoard {
    BitBoard(rng.gen::<u64>() & rng.gen::<u64>() & rng.gen::<u64>())
}

// Given a square and the type of piece, lookup the RAYS and remove the endpoint squares.
pub fn magic_mask(square: Square, sliding_direction: SlidingDirection) -> BitBoard {
    RaysGenerator::get_rays(square, sliding_direction)
        & match sliding_direction {
            // We don't include the borders for Bishop attack maps
            SlidingDirection::Diagonal => BitBoard::NOT_BORDERS,
            SlidingDirection::Orthogonal => !Square::ALL
                .iter()
                .filter(|edge| {
                    (square.get_rank() == edge.get_rank()
                        && (edge.get_file() == File::A || edge.get_file() == File::H))
                        || (square.get_file() == edge.get_file()
                            && (edge.get_rank() == Rank::First || edge.get_rank() == Rank::Eighth))
                })
                .fold(BitBoard::EMPTY, |b, s| b | BitBoard::from(*s)),
        }
}

// Given a bitboard, generate a list of every possible set of bitboards using those bits.
// AKA, if 'n' bits are set, generate 2^n bitboards where b1|b2|b3|...b(2^n) == mask
fn rays_to_questions(mask: BitBoard) -> Vec<BitBoard> {
    let mut result = vec![];
    let squares = mask.collect::<Vec<_>>();

    for i in 0..(1u64 << mask.count_bits()) {
        let mut current = BitBoard::EMPTY;
        for j in 0..mask.count_bits() {
            if (i & (1u64 << j)) == (1u64 << j) {
                current |= BitBoard::from_square(squares[j as usize]);
            }
        }
        result.push(current);
    }

    return result;
}

/// Generate all the possible combinations of blocking pieces for the rook/bishop, and then
/// generate all possible moves for each set of blocking pieces.
pub fn questions_and_answers(sq: Square, sliding_direction: SlidingDirection) -> (Vec<BitBoard>, Vec<BitBoard>) {
    let mask = magic_mask(sq, sliding_direction);
    let questions = rays_to_questions(mask);

    let mut answers = vec![];

    let movement = match sliding_direction {
        SlidingDirection::Orthogonal => ROOK_TRANSLATIONS,
        SlidingDirection::Diagonal => BISHOP_TRANSLATIONS,
    };

    for question in questions.iter() {
        let mut answer = BitBoard::EMPTY;
        for m in movement.iter() {
            let mut next = m(sq);
            while next != None {
                answer ^= BitBoard::from_square(next.unwrap());
                if (BitBoard::from_square(next.unwrap()) & *question) != BitBoard::EMPTY {
                    break;
                }
                next = m(next.unwrap());
            }
        }
        answers.push(answer);
    }

    return (questions, answers);
}
