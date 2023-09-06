"use client";

import type { FC } from "react";
import { rspc } from "@acme/server";
import type { ChessJSON, Piece } from "@acme/server/ts/types";
import Square from "./Square";
import * as chessBoard from './utils';
import { getPromotion } from "./pawnPromotion";

export { chessBoard };

const ChessBoard: FC = ({ }) => {
  return (
    <div id={chessBoard.DOM.ID.board} className="grid grid-cols-8 w-full max-w-[52rem]">
      {[...chessBoard.RANKS].reverse().map((rank) => chessBoard.FILES.map((file) => (
        <Square
          key={chessBoard.Square.coords.toId(file, rank)}
          file={file}
          rank={rank}
        />
      )
      ))}
    </div>

  )
}

export default ChessBoard;


export const useChessboard = (gameData: ChessJSON) => {
  const rspcCtx = rspc.useContext();

  const moveMutation = rspc.useMutation('chess.move', {
    onSuccess(data, _variables, _context) {
      rspcCtx.queryClient.setQueryData(['chess.start'], data);
    },
  });

  return {
    squares: gameData.board,
    move: async (origin: chessBoard.Square.Id, destination: chessBoard.Square.Id, piece: Piece) => {
      const promotion = (piece === 'Pawn' && !!gameData.moves.find(
        move => (move.promotion !== null) && move.origin === origin && move.destination === destination
      ))
        ? await getPromotion(destination, gameData.teamToMove).catch(() => 'canceled' as 'canceled')
        : null

      if (promotion === 'canceled') return promotion;

      return moveMutation.mutateAsync([
        {
          origin,
          destination,
          takes: gameData.moves.find(move => move.origin === origin && move.destination === move.destination)?.takes ?? false,
          piece,
          promotion,
        },
        gameData
      ])
    },
  } as const;
}


