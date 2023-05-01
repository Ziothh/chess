"use client";

import { FC } from "react";
import { rspc } from "@acme/server";
import { ChessJSON, ChessPieceVariant } from "@acme/server/ts/bindings";
import { chessBoard } from ".";
import Square from "./Square";


const getSquareId = <F extends chessBoard.File, R extends chessBoard.Rank>(file: F, rank: R) => `${file}${rank}` as `${typeof file}${typeof rank}`

const ChessBoard: FC = ({ }) => {
  return (
    <div className="grid grid-cols-8 w-full max-w-4xl">
      {[...chessBoard.ranks].reverse().map((rank, ri) => chessBoard.files.map((file, fi) => {
        const id = getSquareId(file, rank);

        return (
          <Square
            key={id}
            id={id}
            file={file}
            rank={rank}
            index={(7 - ri) * 8 + fi}
          />
        )
      }))}
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
  })

  return {
    squares: gameData.board,
    move: (origin: chessBoard.SquareId, destination: chessBoard.SquareId, piece: ChessPieceVariant) => moveMutation.mutateAsync([
      {
        origin,
        destination,
        takes: false,
        piece,
      },
      gameData
    ]),
    take: (origin: chessBoard.SquareId, destination: chessBoard.SquareId, piece: ChessPieceVariant) => moveMutation.mutateAsync([
      {
        origin,
        destination,
        takes: true,
        piece,
      },
      gameData
    ]),
  }
}
