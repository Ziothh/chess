# Bitboard attack generation
## Leaper pieces
No special edge cases. Just assign the attacked squares

## Sliding pieces
Attacks: don't include the outer most square on the ray
Attacks on the fly: use a blocker `Bitboard` to calculate where the rays stop (including the occupied square).
These do include the outer most square on the ray (if not blocked)
