/* eslint-disable */
// This file was generated by [rspc](https://github.com/oscartbeaumont/rspc). Do not edit this file manually.

export type Procedures = {
    queries: 
        { key: "chess.start", input: never, result: ChessJSON } | 
        { key: "echo", input: string, result: string },
    mutations: 
        { key: "chess.move", input: [MoveJSON, ChessJSON], result: ChessJSON },
    subscriptions: never
};

export type Team = "Black" | "White"

export type ChessPiece = { team: Team; variant: ChessPieceVariant }

export type ChessPieceVariant = "Pawn" | "Bishop" | "Knight" | "Rook" | "Queen" | "King"

export type ChessJSON = { teamToMove: Team; moves: MoveJSON[]; board: ChessBoard }

export type MoveJSON = { origin: string; destination: string; takes: boolean; piece: ChessPieceVariant }

export type ChessBoard = (ChessPiece | null)[]
