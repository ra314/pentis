// ============================================================================
// PENTOMINOS MODULE
// ============================================================================

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

pub fn get_pieces() -> HashMap<PieceType, HashMap<Rotation, &'static str>> {
    let mut pieces = HashMap::new();
    
    // F piece
    let mut f_piece = HashMap::new();
    f_piece.insert(Rotation::Rot0, ".00\n00.\n.0.");
    f_piece.insert(Rotation::Rot90, ".0.\n000\n..0");
    f_piece.insert(Rotation::Rot180, ".0.\n.00\n00.");
    f_piece.insert(Rotation::Rot270, "0..\n000\n.0.");
    pieces.insert(PieceType::F, f_piece);
    
    // I piece
    let mut i_piece = HashMap::new();
    i_piece.insert(Rotation::Rot0, ".0.\n.0.\n.0.\n.0.\n.0.");
    i_piece.insert(Rotation::Rot90, ".....\n00000\n.....");
    i_piece.insert(Rotation::Rot180, ".0.\n.0.\n.0.\n.0.\n.0.");
    i_piece.insert(Rotation::Rot270, ".....\n00000\n.....");
    pieces.insert(PieceType::I, i_piece);
    
    // L piece
    let mut l_piece = HashMap::new();
    l_piece.insert(Rotation::Rot0, ".0..\n.0..\n.0..\n.00.");
    l_piece.insert(Rotation::Rot90, "....\n0000\n0...\n....");
    l_piece.insert(Rotation::Rot180, ".00.\n..0.\n..0.\n..0.");
    l_piece.insert(Rotation::Rot270, "....\n...0\n0000\n....");
    pieces.insert(PieceType::L, l_piece);
    
    pieces
}
