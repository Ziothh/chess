#[derive(Debug, PartialEq)]
pub enum Team {
    Black,
    White,
}

impl From<char> for Team {
    // ! This is not optimal or safe
    fn from(value: char) -> Self {
        if value.is_uppercase() {
            Self::White
        } else {
            Self::Black
        }
    }
}
