import type { FC } from "react";
import { chessBoard } from ".";
import { useChess } from "../ChessContext";
import { useDraggingPiece } from "../DraggingPieceContext";
import clsx from "clsx";
import { PieceIcon } from "./Piece";

const Square: FC<{
  file: chessBoard.File,
  rank: chessBoard.Rank,
  className?: string,
}> = ({ className, file, rank }) => {
  const id = chessBoard.Square.coords.toId(file, rank);
  const index = chessBoard.Square.getIndexById(id);

  const chess = useChess();
  const draggingPiece = useDraggingPiece()

  /** Piece on this square */
  const piece = chessBoard.getByIndex(chess.board.squares, index)

  const moveToThisSquare = draggingPiece.value?.moves.find(m => m.destination === id) ?? null;

  return (
    <div
      id={chessBoard.DOM.ID.square(id)}
      data-index={index}
      className={clsx(
        "relative flex justify-center items-center aspect-square user-select-none",
        file.charCodeAt(0) % 2 !== rank.charCodeAt(0) % 2 ? 'bg-zinc-400' : 'bg-zinc-950',
        !draggingPiece.isSet
          ? piece?.team === chess.teamToMove && 'cursor-grab'
          : (
            (moveToThisSquare && 'cursor-move')
            || (draggingPiece.value.index === index && 'cursor-not-allowed')
            || (piece && 'cursor-no-drop')
            || 'cursor-grabbing'
          ),
        className,
      )}
      onClick={() => {
        /// [Drag & Move logic]
        
        // If NO dragging piece
        if (draggingPiece.isSet === false) {
          /// Drag the clicked piece
          if (piece?.team === chess.teamToMove) return draggingPiece.setByIndex(index)
          /// Clicking void
          else return;
        }

        // If dragging piece
        /// Toggle dragging
        if (draggingPiece.value.index === index) return draggingPiece.unset();
        draggingPiece.value.squareId

        const moves = draggingPiece.value.moves.filter(m => m.destination === chessBoard.SQUARES[index]);

        // Switch piece
        if (piece?.team === draggingPiece.value.team) return draggingPiece.setByIndex(index);

        // Void click
        if (moves.length === 0) return draggingPiece.unset();

        // Make the move
        return chess.board.move(draggingPiece.value.squareId!, id, draggingPiece.value.variant);
      }}
    >
      {piece && (
        <PieceIcon piece={piece} className={clsx(
          'relative z-10 pointer-events-none',
          draggingPiece.value?.index === index && 'bg-red-700'
        )} />
      )}

      {/* Rank indicators */ file === 'a' && (
        <span className="absolute left-0 top-0 leading-none block p-2 font-bold opacity-50 pointer-events-none">{rank}</span>
      )}
      {/* File indicators */ rank === '1' && (
        <span className="absolute right-0 bottom-0 leading-none block p-2 font-bold opacity-50 pointer-events-none">{file}</span>
      )}

      {moveToThisSquare && (
        <div className={clsx("absolute inset-0 z-10 pointer-events-none",
          moveToThisSquare.takes
            ? 'p-2'
            : 'p-10'
        )}>
          <div className={clsx(
            "w-full h-full rounded-full",
            moveToThisSquare.takes
              ? 'border-zinc-600/30 border-[.5rem]'
              : 'bg-zinc-600/30'
          )} />
        </div>
      )}
    </div>
  )
}

export default Square;
