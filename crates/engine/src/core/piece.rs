use super::team::Team;

#[derive(Debug, PartialEq)]
pub enum ChessPieceVariant {
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

        match value.to_ascii_uppercase() {
            'P' => Ok(Pawn),
            'B' => Ok(Bishop),
            'N' => Ok(Knight),
            'R' => Ok(Rook),
            'Q' => Ok(Queen),
            'K' => Ok(King),
            _ => Err("\"{value}\" is not a valid chess piece representation".to_owned())
        }
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
    team: Team,
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

        if self.team == Team::Black {
            char = char.to_lowercase();
        }

        return char;
    }
}

impl TryFrom<char> for ChessPiece {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(Self {
          team: Team::from(value),
          variant: ChessPieceVariant::try_from(value)?
        })
    }
}
