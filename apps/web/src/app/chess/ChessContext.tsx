"use client";

import { contextFactory } from "~/utils/components";
import { DraggingPieceProvider, useDraggingPiece } from "./DraggingPieceContext";
import { rspc } from "@acme/server";
import { Suspense } from "react";
import { useChessboard } from "./ChessBoard";

export const [useChess, ChessCtxProvider] = contextFactory(
  () => {
    const { data } = rspc.useQuery(['chess.start'], {
      suspense: true,
      staleTime: Infinity,
      refetchOnMount: false,
      refetchOnWindowFocus: false,
      refetchOnReconnect: false,
      // initialData: {
      //   board: new Array(64).fill(null),
      //   team_to_move: 'White',
      //   fullmove_clock: 0,
      //   halfmove_clock: 0,
      // }
    })

    if (data === undefined) { throw new Error("The chess.start data can not be undefined") }
    const { board: _, moves, teamToMove } = data

    const board = useChessboard(data)

    return {
      moves,
      teamToMove,
      board,
    };
  },
  ({ children }) => {
    return (
      <Suspense fallback={<h1>Loading...</h1>}>
        {children}
      </Suspense>
    )
  }
)

export type ChessContext = ReturnType<typeof useChess>;
