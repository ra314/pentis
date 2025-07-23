// ============================================================================
// MAIN - DEMO/TEST
// ============================================================================

fn main() {
    println!("Pentomino Game - Rust Version");
    
    // Create a board
    let mut board = Board::create();
    
    // Create a piece
    let piece = Piece::create(PieceType::F, false);
    
    // Add piece to board
    board.add_piece(piece);
    
    // Display initial state
    board.display();
    
    // Drop the piece a few times
    for i in 0..5 {
        println!("\n--- Drop {} ---", i + 1);
        board.drop();
        board.display();
        
        if board.current_piece.is_none() {
            println!("Piece stopped!");
            break;
        }
    }
    
    // Test different pieces
    println!("\n=== Testing Different Pieces ===");
    
    let pieces_to_test = [PieceType::I, PieceType::L];
    
    for piece_type in pieces_to_test {
        println!("\n--- Testing {:?} piece ---", piece_type);
        let mut test_board = Board::create();
        let test_piece = Piece::create(piece_type, false);
        test_board.add_piece(test_piece);
        test_board.display();
        
        // Test rotation
        if let Some(ref mut piece) = test_board.current_piece {
            piece.pivot(true);
            println!("After rotation:");
            test_board.display();
        }
    }
}
