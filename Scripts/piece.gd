extends Node2D

class_name Piece

static func create(piece_enum: GLOBALS.PIECE, is_mirrored: bool) -> Piece:
	Piece.color = null
	return null

func pivot(clockwise: bool) -> void:
	rot = GLOBALS.pivot(rot, clockwise)
	# TODO
	# Apply rotation to board state, fail if not possible.

func get_vectors() -> Array[Vector2i]:
	return Numpy.add(Pentominos.parse_piece(Pentominos.PIECES[piece_enum][rot]), cur_pos)

func drop() -> void:
	cur_pos += Vector2i(0, 1)

func undrop() -> void:
	cur_pos += Vector2i(0, -1)

var cur_pos := Vector2i()
var rot := GLOBALS.ROTATION.ROT0
var is_mirrored := false
var piece_enum: GLOBALS.PIECE
var color: Color
