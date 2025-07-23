use crate::{globals::{BOARD_COLS, BOARD_ROWS}, piece::Piece};

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
            colors[i].resize(BOARD_ROWS, None);
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
