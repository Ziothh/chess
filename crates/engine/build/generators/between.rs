use crate::{bitboard::BitBoard, primitives::Square};
use super::prelude::{ArrayGenerator, square_with_i8_coords};


/// Returns `true` if `x` is in the range of `]min(a, b), max(a, b)[`, else `false`
fn is_between(a: i8, x: i8, b: i8) -> bool {
    if a < b {
        a < x && x < b
    } else {
        b < x && x < a
    }
}


/// A `ArrayGenerator` that generates the lines between 2 squares
pub struct BetweenGenerator;

impl ArrayGenerator<[BitBoard; 64]> for BetweenGenerator {
    const NAME: &'static str = "BETWEEN";

    // Will loop over all 64 squares
    fn generate_index_value(index: usize) -> [BitBoard; 64] {
        let origin = Square::new(index as u8);
        let (origin, origin_file, origin_rank) = square_with_i8_coords(&origin);

        return Square::ALL
            .iter()
            .map(square_with_i8_coords)
            .map(|(dest, dest_file, dest_rank)| Square::ALL
                .iter()
                .filter(|test| {
                    let (_, test_file, test_rank) = square_with_i8_coords(*test);

                    // Diagonal
                    if (origin_file - dest_file).abs() == (origin_rank - dest_rank).abs()
                        && *origin != *dest
                    {
                        return (origin_file - test_file).abs() == (origin_rank - test_rank).abs()
                            && (dest_file - test_file).abs() == (dest_rank - test_rank).abs()
                            && is_between(origin_rank, test_rank, dest_rank);
                    }
                    // Check Rank and File lines (horizontal & vertical)
                    else if (origin_file == dest_file || origin_rank == dest_rank)
                        && *origin != *dest
                    {
                        return 
                            // Same file
                            (origin_file == test_file && dest_file == test_file && is_between(origin_rank, test_rank, dest_rank))
                            // Same rank
                            || (origin_rank == test_rank && dest_rank == test_rank && is_between(origin_file, test_file, dest_file));
                    }
                    // No lines found, so filter this one out
                    else {
                        return false;
                    }

                })
                .fold(BitBoard::EMPTY, |bb, square| bb | BitBoard::from(*square))
            )
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
    }
}
