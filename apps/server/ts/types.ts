export * from './bindings';

// import type { ChessBoard as GeneratedChessBoard, ChessJSON as GeneratedChessJSON } from './bindings'
//
// type Tuple<T, N extends number> = N extends N ? number extends N ? T[] : _TupleOf<T, N, []> : never;
// type _TupleOf<T, N extends number, R extends unknown[]> = R['length'] extends N ? R : _TupleOf<T, N, [T, ...R]>;
//
// export type ChessBoard = Tuple<GeneratedChessBoard[number], 64>;
//
// export type ChessJSON = prettyfy<Omit<GeneratedChessJSON, 'board'> & {
//   board: ChessBoard;
// }> 
//
//
// // 
// type prettyfy<T> = { [K in keyof T]: T[K] };
