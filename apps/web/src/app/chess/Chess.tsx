"use client";

import { ChessCtxProvider, useChess } from "./ChessContext";
import ChessBoard from "./ChessBoard";
import { DraggingPieceProvider } from "./DraggingPieceContext";
import { withWrapper } from "~/utils/components";
import { EventHistory } from "~/features";

const Chess = withWrapper(ChessCtxProvider, () => {
  const chess = useChess();

  // console.log(chess);

  return (
    <div className="flex justify-center gap-4 py-8 w-full px-16">
      <div className="flex flex-col items-center gap-4">
        <h1 className="text-3xl font-bold">Chess - {chess.teamToMove}</h1>
        <DraggingPieceProvider>
          <ChessBoard />
        </DraggingPieceProvider>
      </div>

      <div className="rounded-md border-2 border-zinc-900 py-2 px-4 w-full max-w-xs">
        <h2 className="text-2xl font-bold">Chat</h2>
      </div>
    </div>
  )
})

export default Chess;
