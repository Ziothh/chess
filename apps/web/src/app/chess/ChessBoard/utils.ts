import { ChessBoard } from "@acme/server/ts/bindings";

export const FILES = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] as const;
export const RANKS = ['1', '2', '3', '4', '5', '6', '7', '8'] as const;

export const RANKED_FILES = RANKS.map(r => FILES.map(f => `${f}${r}` as `${typeof f}${typeof r}`));
export const SQUARES = RANKED_FILES.flat();

export type Rank = typeof RANKS[number];
export type File = typeof FILES[number];

export const getByIndex = (board: ChessBoard, index: number) => board[validateIndex(index)] as ChessBoard[number];
export const getById = (board: ChessBoard, id: Square.Id) => getByIndex(board, Square.getIndexById(id));

export namespace Square {
  export type Index = Range<0, 65>;
  export type Id = typeof SQUARES[number];

  export const getIdByIndex = (index: number) => SQUARES[validateIndex(index)]!;
  export const getIndexById = (id: Square.Id) => validateIndex(SQUARES.findIndex(sqId => id === sqId));

  export const coords = {
    toIndex: (file: File, rank: Rank) => (7 - RANKS.indexOf(rank)) * 8 + FILES.indexOf(file),
    toId: <F extends File, R extends Rank>(file: F, rank: R) => `${file}${rank}` as `${typeof file}${typeof rank}`
  } as const;
}



// Utils
type Enumerate<N extends number, Acc extends number[] = []> = Acc['length'] extends N
  ? Acc[number]
  : Enumerate<N, [...Acc, Acc['length']]>

type Range<F extends number, T extends number> = Exclude<Enumerate<T>, Enumerate<F>>


const validateIndex = (index: number): Square.Index => {
  if (index < 0 || index > 64) throw new Error(`Square index ${index} is not inside the range [0, 64]`)
  return index as Square.Index;
}
