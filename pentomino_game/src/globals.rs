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
