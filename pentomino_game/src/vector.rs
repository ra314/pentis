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
