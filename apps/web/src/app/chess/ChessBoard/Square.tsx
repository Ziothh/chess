import { FC } from "react";
import { chessBoard } from ".";
import { useChess } from "../ChessContext";
import { useDraggingPiece } from "../DraggingPieceContext";
import clsx from "clsx";


const Square: FC<{
  file: chessBoard.File,
  rank: chessBoard.Rank,
  className?: string,
}> = ({ className, file, rank }) => {
  const id = chessBoard.Square.coords.toId(file, rank);
  const index = chessBoard.Square.getIndexById(id);

  const chess = useChess();
  const draggingPiece = useDraggingPiece()

  const piece = chessBoard.getByIndex(chess.board.squares, index)

  const moveToThisSquare = draggingPiece.moves?.find(m => m.destination === id) ?? null;

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
        if (draggingPiece.isSet === false) {
          /// Drag the clicked piece
          if (piece?.team === chess.teamToMove) return draggingPiece.setByIndex(index)
          /// Clicking void
          else return;
        }

        // If dragging piece
        /// Toggle dragging
        if (draggingPiece.index === index) return draggingPiece.unset();

        const move = draggingPiece.moves.find(m => m.destination === chessBoard.SQUARES[index]);

        if (!move && !piece) return draggingPiece.unset();

        if (piece) {
          // Can't take own team
          if (piece.team === draggingPiece.team) return draggingPiece.setByIndex(index);

          // Take this square
          if (move?.takes) return chess.board.take(draggingPiece.squareId, id, draggingPiece.variant);
        } else {
          if (!move) return draggingPiece.unset();


          return chess.board.move(draggingPiece.squareId!, id, draggingPiece.variant);
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
        <div className={clsx("absolute inset-0 z-10 pointer-events-none",
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
