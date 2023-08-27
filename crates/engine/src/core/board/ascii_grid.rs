// TODO: make a util function to create an ascii grid
// e.g. fn ascii_grid(fn: (file: File, rank: Rank) -> impl ToString) -> String

// fn ascii_grid_inspiration() {
//     let string = Rank::ALL
//         .iter()
//         .map(|rank| {
//             File::ALL
//                 .iter()
//                 .map(|file| {
//                     mask_attacks(Square::make_square(*file, *rank))
//                         .count_bits()
//                         .to_string()
//                 })
//                 .collect::<Vec<_>>()
//                 .join(" ")
//         })
//         .collect::<Vec<_>>()
//         .join("\n");
//
//     assert_eq!(
//         string,
//         [
//             "6 5 5 5 5 5 5 6",
//             "5 5 5 5 5 5 5 5",
//             "5 5 7 7 7 7 5 5",
//             "5 5 7 9 9 7 5 5",
//             "5 5 7 9 9 7 5 5",
//             "5 5 7 7 7 7 5 5",
//             "5 5 5 5 5 5 5 5",
//             "6 5 5 5 5 5 5 6",
//         ]
//         .join("\n")
//     );
// }
