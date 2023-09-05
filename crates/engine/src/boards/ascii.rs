use std::fmt::Display;

use crate::primitives::{File, Rank, Square};

use super::ChessBoard;

// pub struct ASCIIGrid(String);
//
// impl ASCIIGrid {
//     pub fn new(get_board_data: impl Fn(File, Rank) -> &'static str) -> Self {
//         Self(
//             Rank::ALL
//                 .iter()
//                 .map(|rank| {
//                     File::ALL
//                         .iter()
//                         .map(|file| get_board_data(*file, *rank))
//                         .collect::<Vec<_>>()
//                         .join(" ")
//                 })
//                 .collect::<Vec<_>>()
//                 .join("\n"),
//         )
//     }
// }
//
// impl ToString for ASCIIGrid {
//     fn to_string(&self) -> String {
//         self.0.to_owned()
//     }
// }

pub struct ASCIIBoard {
    // pub board: ChessBoard,
    // pub marks: [bool; ChessBoard::SIZE],
    title: Option<String>,
    square_marks: [char; File::SIZE * Rank::SIZE],
}

impl Default for ASCIIBoard {
    fn default() -> Self {
        Self {
            title: None,
            square_marks: ['.'; File::SIZE * Rank::SIZE],
        }
    }
}

impl ASCIIBoard {
    // pub fn new() -> Self {
    //     Self {
    //         // board,
    //         marks: Self::DEFAULT_MARKS,
    //     }
    // }
    pub fn title(&mut self, title: impl ToString) -> &mut Self {
        self.title = Some(title.to_string());

        return self;
    }

    pub fn set(&mut self, char: char, square: Square) -> &mut Self {
        self.square_marks[square.to_index()] = char;

        return self;
    }
}

impl Display for ASCIIBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(title) = self.title.to_owned() {
            write!(f, "\n   {title}\n\n")?;
        }

        for rank in Rank::ALL.iter().rev() {
            write!(f, "{rank} ")?;

            for file in File::ALL.iter() {
                write!(f, " ")?;

                let square = Square::make_square(*file, *rank);

                write!(f, "{}", self.square_marks[square.to_index()])?;
            }

            write!(f, "\n")?;
        }

        write!(
            f,
            "\n   {}",
            File::ALL
                .iter()
                .map(|file| file.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )?;

        return Ok(());
    }
}
