# Emerald
A UCI chess engine written in Rust.

## About

Emerald has support for basic UCI commands, however the implementation is not complete and the engine does not support every command.
Emerald also has a few non-UCI commands:
1.    ``perftsuite``: Runs the perft suite ethereal.
2.    ``perft <depth>``: Runs a perft test to the specified depth.
3.   ``splitperft <depth>``: Runs a perft where each move is also printed with amount of nodes after playing the move.

### Board Representation

Emerald uses 8 bitboards into to represent all of the pieces, 2 for each colour, and 6 for each type of piece. <br>
Emerald also uses magic bitboards in order to generate piece moves for sliding pieces (rook, bishop and queen) and lookup tables for the other pieces.<br>
Emerald also generates pseudolegal moves and uses copymake moves.