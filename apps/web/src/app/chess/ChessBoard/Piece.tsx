import type { FC } from "react";
import { chessBoard } from ".";
import { useChess } from "../ChessContext";
import type { ChessPiece } from "@acme/server/ts/bindings";

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

interface Props {
  className?: string,
  piece: ChessPiece,
  // team: Team,
}

export const PieceIcon: FC<Props> = ({ piece, className }) => (
  <img
    className={className}
    src={`/${piece.team.at(0)?.toLowerCase()}${piece.variant === "Knight" ? "n" : piece.variant.at(0)?.toLowerCase()}.png`}
  />
);
