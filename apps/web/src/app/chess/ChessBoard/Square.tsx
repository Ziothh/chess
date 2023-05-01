import { FC } from "react";
import { chessBoard } from ".";
import { useChess } from "../ChessContext";
import { useDraggingPiece } from "../DraggingPieceContext";
import clsx from "clsx";
import { squares } from "./config";


const Square: FC<{
  id: chessBoard.SquareId,
  file: chessBoard.File,
  rank: chessBoard.Rank,
  index: number,
  className?: string,
}> = ({ id, index, className, file, rank }) => {
  const chess = useChess();
  const piece = chess.board.squares.at(index)
  const draggingPiece = useDraggingPiece()

  const moveToThisSquare = draggingPiece.value?.moves.find(m => m.destination === chessBoard.squares[index]) ?? null;

  return (
    <div
      id={id}
      data-index={index}
      className={clsx(
        file.charCodeAt(0) % 2 !== rank.charCodeAt(0) % 2 ? 'bg-slate-400' : 'bg-slate-800',
        "relative aspect-square cursor-pointer user-select-none",
        className,
      )}
      onClick={() => {
        // If NO dragging piece
        if (draggingPiece.value === null || draggingPiece.index === null) {
          /// Drag the clicked piece
          if (piece?.team === chess.teamToMove) return draggingPiece.setIndex(index)
          /// Clicking void
          else return;
        }

        // If dragging piece
        /// Toggle dragging
        if (draggingPiece.index === index) return draggingPiece.remove();


        const move = draggingPiece.value?.moves.find(m => m.destination === squares[index]);

        if (!move && !piece) return draggingPiece.remove();

        if (piece) {
          // Can't take own team
          if (piece.team === draggingPiece.value.team) return draggingPiece.setIndex(index);

          // Take this square
          if (move?.takes) return chess.board.take(draggingPiece.square!, id, draggingPiece.value.variant);
        } else {
          if (!move) return draggingPiece.remove();


          return chess.board.move(draggingPiece.square!, id, draggingPiece.value.variant);
        }
      }}
    >
      {piece && (
        <img
          className={clsx(
            'relative z-10 pointer-events-none',
            draggingPiece.index === index && 'bg-red-500'
          )}
          src={`/${piece.team.at(0)?.toLowerCase()}${piece.variant === "Knight" ? "n" : piece.variant.at(0)?.toLowerCase()}.png`}
        />
      )}
      {file === 'a' && (
        <span className="absolute left-0 top-0 leading-none block p-2 font-bold opacity-50 pointer-events-none">{rank}</span>
      )}
      {rank === '1' && (
        <span className="absolute right-0 bottom-0 leading-none block p-2 font-bold opacity-50 pointer-events-none">{file}</span>
      )}

      {moveToThisSquare && (
        <div className={clsx("absolute inset-0 z-10",
          moveToThisSquare.takes
            ? 'p-2'
            : 'p-10'
        )}>
          <div className={clsx(
            "w-full h-full rounded-full",
            moveToThisSquare.takes
              ? 'border-black/25 border-[.5rem]'
              : 'bg-black/25'
          )} />
        </div>
      )}
    </div>
  )
}

export default Square;
