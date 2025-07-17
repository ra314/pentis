// Cargo.toml
/*
[package]
name = "pentomino_game"
version = "0.1.0"
edition = "2021"

[dependencies]
*/

use std::collections::HashMap;

// ============================================================================
// GLOBALS MODULE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Unset,
    F,
    I,
    L,
    N,
    P,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

pub const MIRRORABLE_PIECES: [PieceType; 6] = [
    PieceType::F,
    PieceType::L,
    PieceType::N,
    PieceType::P,
    PieceType::Y,
    PieceType::Z,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rotation {
    Rot0,
    Rot90,
    Rot180,
    Rot270,
}

impl Rotation {
    pub fn values() -> [Rotation; 4] {
        [Rotation::Rot0, Rotation::Rot90, Rotation::Rot180, Rotation::Rot270]
    }
}

// Board Dimensions
pub const BOARD_ROWS: usize = 20;
pub const BOARD_COLS: usize = 10;

pub fn pivot(rot: Rotation, clockwise: bool) -> Rotation {
    let rotations = Rotation::values();
    let current_index = rotations.iter().position(|&r| r == rot).unwrap();
    
    let next_index = if clockwise {
        (current_index + 1) % rotations.len()
    } else {
        (current_index + rotations.len() - 1) % rotations.len()
    };
    
    rotations[next_index]
}

// ============================================================================
// VECTOR2I (Position)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2i {
    pub x: i32,
    pub y: i32,
}

impl Vector2i {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl std::ops::Add for Vector2i {
    type Output = Vector2i;
    
    fn add(self, rhs: Vector2i) -> Vector2i {
        Vector2i {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vector2i {
    fn add_assign(&mut self, rhs: Vector2i) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// ============================================================================
// NUMPY MODULE (Vector operations)
// ============================================================================

pub fn add_vectors(slice: &[Vector2i], vec: Vector2i) -> Vec<Vector2i> {
    slice.iter().map(|&x| x + vec).collect()
}

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

// ============================================================================
// BOARD STRUCT
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    White,
}

pub struct Board {
    pub colors: Vec<Vec<Option<Color>>>,
    pub current_piece: Option<Piece>,
}

impl Board {
    pub fn create() -> Self {
        let mut colors = Vec::new();
        colors.resize(BOARD_ROWS, Vec::new());
        
        for i in 0..BOARD_ROWS {
            colors[i].resize(BOARD_COLS, None);
        }
        
        Self {
            colors,
            current_piece: None,
        }
    }
    
    pub fn add_piece(&mut self, piece: Piece) {
        self.current_piece = Some(piece);
    }
    
    pub fn stop_drop(&mut self) {
        if let Some(ref mut piece) = self.current_piece {
            piece.undrop();
        }
        self.current_piece = None;
    }
    
    pub fn drop(&mut self) {
        if let Some(ref mut piece) = self.current_piece {
            piece.drop();
            
            for vec in piece.get_vectors() {
                if vec.y >= BOARD_ROWS as i32 {
                    self.stop_drop();
                    return;
                }
                
                if vec.y >= 0 && vec.x >= 0 && 
                   vec.y < BOARD_ROWS as i32 && vec.x < BOARD_COLS as i32 {
                    if self.colors[vec.y as usize][vec.x as usize].is_some() {
                        self.stop_drop();
                        return;
                    }
                }
            }
        }
    }
    
    pub fn display(&self) {
        println!("\n=== BOARD ===");
        
        // Create a temporary display board that includes the current piece
        let mut display_board = self.colors.clone();
        
        // If there's a current piece, overlay it on the display
        if let Some(ref piece) = self.current_piece {
            for vec in piece.get_vectors() {
                if vec.y >= 0 && vec.x >= 0 && 
                   vec.y < BOARD_ROWS as i32 && vec.x < BOARD_COLS as i32 {
                    display_board[vec.y as usize][vec.x as usize] = Some(Color::Red);
                }
            }
        }
        
        // Display the board with the current piece
        for row in display_board {
            for cell in row {
                match cell {
                    Some(Color::Black) => print!("B"),
                    Some(Color::Red) => print!("R"),  // Current piece
                    Some(Color::Green) => print!("G"),
                    Some(Color::Blue) => print!("B"),
                    Some(Color::Yellow) => print!("Y"),
                    Some(Color::Magenta) => print!("M"),
                    Some(Color::Cyan) => print!("C"),
                    Some(Color::White) => print!("W"),
                    None => print!("."),
                }
            }
            println!();
        }
        
        if let Some(ref piece) = self.current_piece {
            println!("Current piece: {:?} at {:?}", piece.piece_enum, piece.cur_pos);
            println!("Piece vectors: {:?}", piece.get_vectors());
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    
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
