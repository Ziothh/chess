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

  return {
    setByIndex: (index: chessBoard.Square.Index) => draggingPieceIndex.set(index),
    ...(
      piece === null
        ? {
          isSet: false as false,
          value: null,
          // ...{} as { [K in keyof ReturnType<typeof dataIfSet>]: undefined },
        }
        : {
          isSet: true as true,
          value: {
            index: draggingPieceIndex.value!,
            moves: piece!.moves,
            team: piece!.team,
            variant: piece!.variant,
            squareId: id!,
          },
          unset: () => draggingPieceIndex.set(null),
        }
    )
  } as const;
});
