"use client";

import type { FC } from "react";
import { rspc } from "@acme/server";
import type { ChessJSON, ChessPieceVariant } from "@acme/server/ts/types";
import Square from "./Square";
import * as chessBoard from './utils';

export { chessBoard };

const ChessBoard: FC = ({ }) => {
  return (
    <div className="grid grid-cols-8 w-full max-w-[52rem]">
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
    move: (origin: chessBoard.Square.Id, destination: chessBoard.Square.Id, piece: ChessPieceVariant) => moveMutation.mutateAsync([
      {
        origin,
        destination,
        takes: false,
        piece,
      },
      gameData
    ]),
    take: (origin: chessBoard.Square.Id, destination: chessBoard.Square.Id, piece: ChessPieceVariant) => moveMutation.mutateAsync([
      {
        origin,
        destination,
        takes: true,
        piece,
      },
      gameData
    ]),
  } as const;
}


