# pentis

TODO:
Rename pentomino.rs to pentomino_parser.
Rewrite the pentomino_parser. We expect the parser to do the mirroring and rotation to generate the necessary pieces.
  Follow the guide in https://remysharp.com/2019/09/10/blocks-of-tetris-code
Create a custom type to hold all possible combinations of piece type, rotation and mirroring.
  Maybe we also compute this on the fly? Potentially faster than a cache miss if our representation is small enough to fit in cache.
  Ideally we shouldn't need a hashmap for this since we know exactly how many piece types there can be.
  The custom type should also allow lookup via piece type and rotation enum, and perform the conversion to retrieve the coordinates from some pre-allocated and computed array.