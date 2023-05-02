import { chessBoard } from ".";
import { useChess } from "../ChessContext";

export const usePiece = (square: chessBoard.Square.Id | null) => {
  const { board, moves } = useChess()

  if (square === null) return null;

  const piece = chessBoard.getById(board.squares, square);

  if (!piece) return null;

  return {
    ...piece,
    moves: moves.filter(m => m.origin === square)
  }
}
