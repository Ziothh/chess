'use client';

import { Piece, Team } from "@acme/server/ts/bindings";
import { FC, createElement } from "react";
import { Square } from "./utils";
import { chessBoard } from ".";
import { Root, createRoot } from "react-dom/client";
import { PieceIcon } from "./Piece";
import { useKeyPress } from "~/hooks";

const PROMOTION_TARGETS = ['Knight', 'Bishop', 'Rook', 'Queen'] as const satisfies readonly Piece[];

const PawnPromotionMenu: FC<{
  /** The `Team` of the pawn to be promoted */
  team: Team,
  /** Callback when a promotion target is clicked */
  onChoose: (piece: Piece) => void,
  /** Callback when pressing Escape or clicking the modal backdrop */
  onCancel: () => void,
}> = ({ team, onChoose, onCancel }) => {
  useKeyPress({ 'Escape': onCancel });

  return (
    <>
      <div aria-hidden
        className="cursor-default fixed z-modal inset-0 bg-zinc-600/25 backdrop-blur-[2px]"
        onClick={(e) => {
          e.stopPropagation();
          e.preventDefault();
          onCancel();
        }}
      />
      <menu className="cursor-default absolute z-modal bottom-full flex gap-2 bg-zinc-950 border border-zinc-900 rounded-md p-2 mb-4 shadow-sm shadow-zinc-900">
        {PROMOTION_TARGETS.map(piece => (
          <li key={piece} className="contents">
            <button
              title={`Promote pawn to ${piece.toLowerCase()}`}
              className="border border-zinc-900 p-1 rounded-md w-16 transition hover:bg-zinc-900"
              onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();
                onChoose(piece)
              }}
            >
              <PieceIcon piece={{
                team,
                variant: piece,
              }} />
            </button>
          </li>
        ))}
      </menu>
    </>
  );
}

let root: Root | null = null;

export const getPromotion = async (square: Square.Id, team: Team) => {
  const squareDiv = chessBoard.DOM.querySelections.square(square);

  const container = document.createElement('div');
  container.className = "flex justify-center items-center absolute left-0 w-full bottom-full"
  squareDiv.appendChild(container);



  return new Promise<Piece>((res, rej) => {
    if (root != null) return rej('A pawn promotion menu is already active');

    root = createRoot(container);

    return root.render(<PawnPromotionMenu team={team} onCancel={() => rej('Canceled')} onChoose={res} />);
  })
    .finally(() => {
      try {
        root?.unmount();
      } catch (_) { }
      try {
        squareDiv?.removeChild(container);
      } catch (_) { }
      try {
        container?.remove();
      } catch (_) { }

      root = null;
    })
}
