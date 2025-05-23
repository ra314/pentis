extends Node2D

class_name Piece

static func create(piece_enum: GLOBALS.PIECE, is_mirrored: bool) -> Piece:
	return null

func pivot(clockwise: bool) -> void:
	rot = GLOBALS.pivot(rot, clockwise)
	# TODO
	# Apply rotation to board state, fail if not possible.

var cur_pos := Vector2i()
var rot := GLOBALS.ROTATION.ROT0
var is_mirrored := false
var piece_enum: GLOBALS.PIECE
