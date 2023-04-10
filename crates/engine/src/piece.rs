#[derive(Debug, PartialEq)]
enum ChessPieceColor {
    Black,
    White,
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

struct ChessPiece {
    color: ChessPieceColor,
    variant: ChessPieceVariant,
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
