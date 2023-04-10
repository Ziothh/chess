#[derive(Debug, PartialEq)]
pub enum ChessPieceColor {
    Black,
    White,
}

impl From<char> for ChessPieceColor {
    fn from(value: char) -> Self {
        if value.is_uppercase() {
            Self::White
        } else {
            Self::Black
        }
    }
}

#[derive(Debug, PartialEq)]
enum ChessPieceVariant {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl TryFrom<char> for ChessPieceVariant {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use self::ChessPieceVariant::*;

        Ok(match value.to_ascii_uppercase() {
            'P' => Pawn,
            'B' => Bishop,
            'N' => Knight,
            'R' => Rook,
            'Q' => Queen,
            'K' => King,
            // TODO: convert this into an error
            _ => unreachable!("\"{value}\" is not a valid chess piece representation"),
        })
    }
}

impl ToString for ChessPieceVariant {
    fn to_string(&self) -> String {
        use ChessPieceVariant::*;
        match self {
            Pawn => 'P',
            Bishop => 'B',
            Knight => 'N',
            Rook => 'R',
            Queen => 'Q',
            King => 'K',
        }
        .to_string()
    }
}

#[derive(Debug, PartialEq)]
pub struct ChessPiece {
    color: ChessPieceColor,
    variant: ChessPieceVariant,
}

impl ChessPiece {
  pub fn is_sliding(&self) -> bool {
    use self::ChessPieceVariant::*;

    match self.variant {
      Bishop => true,
      Rook => true,
      Queen => true,
      _ => false,
    }
  }
    
}

impl ToString for ChessPiece {
    fn to_string(&self) -> String {
        let mut char = self.variant.to_string();

        if self.color == ChessPieceColor::Black {
            char = char.to_lowercase();
        }

        return char;
    }
}

impl TryFrom<char> for ChessPiece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
          color: ChessPieceColor::from(value),
          variant: ChessPieceVariant::try_from(value)?
        })
    }
}
