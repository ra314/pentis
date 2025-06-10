extends Node2D

class_name Board

static func create() -> Board:
	var board := Board.new()

	# Setup the board with null representing no color/piece.
	var c: Array[Array] = board.colors
	c = []
	c.resize(GLOBALS.BOARD_ROWS)

	for i in range(GLOBALS.BOARD_ROWS):
		c[i] = []
		c[i].resize(GLOBALS.BOARD_COLS)
		for j in range(GLOBALS.BOARD_COLS):
			c[i][j] = null

	return board

func addPiece(piece: Piece):
	return null

func drop():
	return null

# Colors has type Array[Array[Color]]
var colors: Array[Array]
