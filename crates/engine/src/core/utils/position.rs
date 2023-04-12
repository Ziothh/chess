use crate::game::CHESSBOARD_WIDTH;


/** Generates a chessboard position string from the position index (0..64).
 * Example: index `56` = `"a7"` = top left corner */
pub fn position_to_string(position: usize) -> String {
    let x = position % CHESSBOARD_WIDTH;
    let y = position / CHESSBOARD_WIDTH;

    return format!(
        "{}{y}",
        char::from_digit((10 + x) as u32, 36).unwrap().to_string()
    );
}

pub fn position_from_string(position: &str) -> usize {
    let mut chars = position.chars();

    let x = chars.next().unwrap() as usize - 97;
    let y = chars.next().unwrap().to_digit(10).unwrap() as usize;

    x + y * CHESSBOARD_WIDTH
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions() {
        (0..64).for_each(|index| {
            let str_repr = position_to_string(index);
            let num_repr = position_from_string(&str_repr);

            assert_eq!(index, num_repr);
        });
    }
}
