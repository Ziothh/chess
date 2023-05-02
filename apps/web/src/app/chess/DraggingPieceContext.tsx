import { useStateObject } from "~/hooks";
import { contextFactory } from "~/utils/components";
import { usePiece } from "./ChessBoard/Piece";
import { chessBoard } from "./ChessBoard";

export const [useDraggingPiece, DraggingPieceProvider] = contextFactory(() => {
  const draggingPieceIndex = useStateObject<chessBoard.Square.Index | null>(null);
  const id = draggingPieceIndex.value === null
    ? null
    : chessBoard.Square.getIdByIndex(draggingPieceIndex.value);

  const piece = usePiece(id);

  const dataIfSet = () => ({
    index: draggingPieceIndex.value!,
    moves: piece!.moves,
    team: piece!.team,
    variant: piece!.variant,
    squareId: id!,

    remove: () => draggingPieceIndex.set(null),
  })

  return {
    setByIndex: (index: chessBoard.Square.Index) => draggingPieceIndex.set(index),
    ...(
      piece === null
        ? {
          ...{} as { [K in keyof ReturnType<typeof dataIfSet>]: undefined },
          isSet: false as false,
        }
        : {
          isSet: true as true,

          index: draggingPieceIndex.value!,
          moves: piece.moves,
          team: piece.team,
          variant: piece.variant,
          squareId: id!,

          unset: () => draggingPieceIndex.set(null),
        }
    )
  }
})
