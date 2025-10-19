// ============================================================================
// PENTOMINOS MODULE
// ============================================================================

use std::collections::HashMap;

use crate::globals::PieceType;
use crate::vector::Vector2i;

pub fn parse_piece(raw_piece: &str) -> Vec<Vector2i> {
    let mut ret = Vec::new();
    let mut y = 0;

    for line in raw_piece.lines() {
        let mut x = 0;
        for char in line.chars() {
            if char == '0' {
                ret.push(Vector2i::new(x, y));
            }
            x += 1;
        }
        y += 1;
    }

    ret
}

pub fn get_pieces() -> HashMap<PieceType, [Vec<Vector2i>; 4]> {
    use std::fs;
    use std::path::Path;

    let mut pieces = HashMap::new();

    // Read the pentominos.txt file
    let txt_path = Path::new("src/pentominos.txt");
    let txt = fs::read_to_string(txt_path).expect("Failed to read pentominos.txt");

    // Split by piece type (lines with only one character)
    let pieces = txt.split_terminator("\n----------\n");
    let mut lines = txt.lines().peekable();
    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.is_empty() { continue; }
        // Piece type line (e.g., "F", "I", "L")
        if line.len() == 1 {
            let piece_type = PieceType::from_str(line).expect("Invalid PieceType in pentominos.txt");
            let mut rotations = Vec::new();
            // Collect next 4 blocks (each block separated by empty line)
            for _ in 0..4 {
                let mut block = String::new();
                while let Some(&next_line) = lines.peek() {
                    if next_line.trim().is_empty() {
                        lines.next();
                        break;
                    }
                    block.push_str(next_line);
                    block.push('\n');
                    lines.next();
                }
                rotations.push(parse_piece(block.trim_end()));
            }
            // Ensure 4 rotations
            assert_eq!(rotations.len(), 4, "Each piece must have 4 rotations");
            pieces.insert(piece_type, [rotations[0].clone(), rotations[1].clone(), rotations[2].clone(), rotations[3].clone()]);
        }
    }

    pieces
}
