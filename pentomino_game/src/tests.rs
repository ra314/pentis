#[cfg(test)]
mod tests {
    use crate::board::Board;
    use crate::globals::{BOARD_COLS, BOARD_ROWS, PieceType, Rotation, pivot};
    use crate::piece::Piece;
    use crate::vector::Vector2i;

    #[test]
    fn test_board_creation() {
        let board = Board::create();
        assert_eq!(board.colors.len(), BOARD_ROWS);
        assert_eq!(board.colors[0].len(), BOARD_COLS);
        assert!(board.current_piece.is_none());
    }

    #[test]
    fn test_piece_creation() {
        let piece = Piece::create(PieceType::F, false);
        assert_eq!(piece.piece_enum, PieceType::F);
        assert_eq!(piece.rot, Rotation::Rot0);
        assert!(!piece.is_mirrored);
        assert_eq!(piece.cur_pos, Vector2i::zero());
    }

    #[test]
    fn test_pivot_rotation() {
        assert_eq!(pivot(Rotation::Rot0, true), Rotation::Rot90);
        assert_eq!(pivot(Rotation::Rot90, true), Rotation::Rot180);
        assert_eq!(pivot(Rotation::Rot180, true), Rotation::Rot270);
        assert_eq!(pivot(Rotation::Rot270, true), Rotation::Rot0);

        assert_eq!(pivot(Rotation::Rot0, false), Rotation::Rot270);
        assert_eq!(pivot(Rotation::Rot270, false), Rotation::Rot180);
    }

    #[test]
    fn test_piece_vectors() {
        let piece = Piece::create(PieceType::F, false);
        let vectors = piece.get_vectors();
        assert!(!vectors.is_empty());

        // F piece at rotation 0 should have 5 blocks
        assert_eq!(vectors.len(), 5);
    }

    #[test]
    fn test_vector_addition() {
        let v1 = Vector2i::new(1, 2);
        let v2 = Vector2i::new(3, 4);
        let result = v1 + v2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }
}
