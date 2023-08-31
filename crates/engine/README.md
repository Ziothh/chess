# Chess engine
## Developer notes
Some possible issues are:
 - black having a high rank value and white a low one
 - - SOLUTION: the convention is that black starts on Rank 7, File A, which is equal to index 0
 - Relevant occupancy bits not working right because I include the A+B & H+G files in the attack maps. 
 - - SOLUTION: read further in the chessprogramming site

## TODO's
 [ ] Reimplement the tests for magic generation (see src/_depricated)
