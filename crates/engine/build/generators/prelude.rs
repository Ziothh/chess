use std::{fmt::Debug, fs::File, io::Write};

use crate::primitives::Square;

/// T: The type of the generated array content
/// N: The size of the generated array
pub trait ArrayGenerator<T: Debug + Sized, const N: usize = 64>
where
    [T; N]: Sized,
{
    /// The name of the `const` array that's generated in the `write_generated_array` function.
    const NAME: &'static str;

    /// The generator function that needs to be implemented.
    /// It is used to generate a value `T` for every `index`.
    fn generate_index_value(index: usize) -> T;

    /// Iterates over every index of the range `[0, N[`,
    /// calls `Self::generate_index_value(index)` for ever every index
    /// and finally collects the values into a `Vec<T>` of size `N`.
    fn generate_values() -> Vec<T> {
        return (0..N)
            .into_iter()
            .map(|index| Self::generate_index_value(index))
            .collect();
    }

    fn generate_array() -> [T; N] {
        return Self::generate_values()
            .try_into()
            .expect("Vector to be of the same length as the expected array");
    }

    fn write_generated_array(file: &mut File) -> std::io::Result<()> {
        let array = Self::generate_array();
        #[rustfmt::skip]
        let type_name = std::any::type_name::<T>()
            .replace("build_script_build::bitboard::bitboard::", "");

        write!(file, "const {}: [{}; 64] = [\n", Self::NAME, type_name)?;
        for i in 0..N {
            write!(file, "    {:?},\n", array[i])?;
        }
        write!(file, "];\n").unwrap();

        return Ok(());
    }
}

/// Returns (square: &Square, file: i8, rank: i8)
pub fn square_with_i8_coords(square: &Square) -> (&Square, i8, i8) {
    return (square, square.get_file() as i8, square.get_rank() as i8);
}
