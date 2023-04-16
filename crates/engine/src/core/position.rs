/// The amount needed to substract from a letter where 'a' = 1, 'b' = 2, ...
// const LOWERCASE_UTF8_OFFSET: usize = 'a' as usize - 1; // If you subsctract from 'a', you get 1
const LOWERCASE_UTF8_OFFSET: usize = 'a' as usize - 1; // If you subsctract from 'a', you get 1

/// The x coord in the chessboard. Starts at 'a' (= 1) and ends at 'h' (= 7)
// type File = usize;
/// The y coord in the chessboard. Starts at 1 and ends at 7
// type Rank = usize;


pub fn rank_char_to_number(rank: &char) -> Result<u32, String> {
  rank.to_digit(10).ok_or("Could not parse rank".to_owned())
}

pub fn file_char_to_number(file: &char) -> usize {
    file - LOWERCASE_UTF8_OFFSET
}

pub fn coords_to_index(file: usize, rank: usize) -> usize {
    ((file - 1) * 8) + rank
}
