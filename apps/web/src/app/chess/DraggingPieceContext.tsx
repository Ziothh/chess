import { useStateObject } from "~/hooks";
import { contextFactory } from "~/utils/components";
import { squares } from "./ChessBoard/config";
import { usePiece } from "./ChessBoard/Piece";

export const [useDraggingPiece, DraggingPieceProvider] = contextFactory(() => {
  const draggingPieceIndex = useStateObject<number | null>(null);

  const _p = usePiece(draggingPieceIndex.value === null ? "a1" : squares[draggingPieceIndex.value]!)
  const piece = draggingPieceIndex.value === null ? null : _p;

  return {
    value: piece,
    index: draggingPieceIndex.value,
    square: draggingPieceIndex.value !== null ? squares[draggingPieceIndex.value]! : null,
    setIndex: (index: number) => draggingPieceIndex.set(index),
    remove: () => draggingPieceIndex.set(null),
  }
})
