// ============================================================================
// PIECE STRUCT
// ============================================================================

pub struct Piece {
    pub cur_pos: Vector2i,
    pub rot: Rotation,
    pub is_mirrored: bool,
    pub piece_enum: PieceType,
    pieces_data: HashMap<PieceType, HashMap<Rotation, &'static str>>,
}

impl Piece {
    pub fn create(piece_enum: PieceType, is_mirrored: bool) -> Self {
        Self {
            cur_pos: Vector2i::zero(),
            rot: Rotation::Rot0,
            is_mirrored,
            piece_enum,
            pieces_data: get_pieces(),
        }
    }
    
    pub fn pivot(&mut self, clockwise: bool) {
        self.rot = pivot(self.rot, clockwise);
        // TODO: Apply rotation to board state, fail if not possible.
    }
    
    pub fn get_vectors(&self) -> Vec<Vector2i> {
        if let Some(piece_rotations) = self.pieces_data.get(&self.piece_enum) {
            if let Some(piece_data) = piece_rotations.get(&self.rot) {
                let parsed = parse_piece(piece_data);
                return add_vectors(&parsed, self.cur_pos);
            }
        }
        Vec::new()
    }
    
    pub fn drop(&mut self) {
        self.cur_pos += Vector2i::new(0, 1);
    }
    
    pub fn undrop(&mut self) {
        self.cur_pos += Vector2i::new(0, -1);
    }
}
