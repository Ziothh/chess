import { chessBoard } from ".";
import { useChess } from "../ChessContext";
import { squares } from "./config";

export const usePiece = (square: chessBoard.SquareId | null) => {
  const { board, moves } = useChess()

  const piece = square === null 
  ? board.squares[squares.findIndex((sq) => sq === square)]
  : null;

  if (!piece) return null;

  return {
    ...piece,
    moves: moves.filter(m => m.origin === square)
  }
}
