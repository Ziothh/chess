"use client";

import { FC } from "react";
import { ChessCtxProvider, useChess } from "./ChessContext";
import ChessBoard from "./ChessBoard";
import { DraggingPieceProvider } from "./DraggingPieceContext";

const Chess: FC = () => {
  const chess = useChess()

  console.log(chess)

  return (
    <div className="flex flex-col items-center py-8 gap-4 w-full">
      <h1 className="text-3xl font-bold">Chess - {chess.teamToMove}</h1>
      <ChessCtxProvider>
        <DraggingPieceProvider>
          <ChessBoard />
        </DraggingPieceProvider>
      </ChessCtxProvider>
    </div>
  )
}

export default Chess;
