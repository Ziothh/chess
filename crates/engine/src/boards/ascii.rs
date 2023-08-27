use crate::core::{File, Rank};

pub struct ASCIIGrid(String);

impl ASCIIGrid {
    pub fn new(get_board_data: impl Fn(File, Rank) -> &'static str) -> Self {
        Self(
            Rank::ALL
                .iter()
                .map(|rank| {
                    File::ALL
                        .iter()
                        .map(|file| get_board_data(*file, *rank))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }
}

impl ToString for ASCIIGrid {
    fn to_string(&self) -> String {
        self.0.to_owned()
    }
}
