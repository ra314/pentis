// ============================================================================
// NUMPY MODULE (Vector operations)
// ============================================================================

pub fn add_vectors(slice: &[Vector2i], vec: Vector2i) -> Vec<Vector2i> {
    slice.iter().map(|&x| x + vec).collect()
}
