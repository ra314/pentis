extends Node2D

class_name Piece

static var piece_enum_to_shape := {GLOBALS.PIECE.F: Pentominos.F, GLOBALS.PIECE.I: Pentominos.I, GLOBALS.PIECE.L: Pentominos.L}

static func create(piece_enum: GLOBALS.PIECE, is_mirrored: bool) -> Piece:
	return null

static func rotate(state: Array[Vector2i]) -> Array[Vector2i]:
	


var cur_pos := Vector2i()
var rot := GLOBALS.ROTATION.ROT0
var is_mirrored := false
