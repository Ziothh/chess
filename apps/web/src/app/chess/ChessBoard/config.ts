export const files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] as const
export const ranks = ['1', '2', '3', '4', '5', '6', '7', '8'] as const

export const rankedFiles = ranks.map(r => files.map(f => `${f}${r}` as `${typeof f}${typeof r}`))
export const squares = rankedFiles.flat()

export type SquareId = typeof squares[number]
export type Rank = typeof ranks[number]
export type File = typeof files[number]
