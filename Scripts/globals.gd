class_name GLOBALS 

enum PIECE {UNSET, F, I, L, N, P, T, U, V, W, X, Y, Z}

const MIRRORABLE_PIECES := [PIECE.F, PIECE.L, PIECE.N, PIECE.P, PIECE.Y, PIECE.Z]

enum ROTATION {ROT0, ROT90, ROT180, ROT270}

static func pivot(rot: ROTATION, clockwise: bool) -> ROTATION:
	var next_rot := rot
	if clockwise:
		next_rot += 1
	else:
		next_rot -= 1
	next_rot %= len(ROTATION)
	return ROTATION.values()[next_rot]
