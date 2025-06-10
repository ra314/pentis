extends Node2D

class_name Board

static var NUM_ROWS = 20
static var NUM_COLS = 10

static func create() -> Board:
	var board := Board.new()

	# Setup the board with null representing no color/piece.
	var c: Array[Array] = board.colors
	c = []
	c.resize(NUM_ROWS)

	for i in range(NUM_ROWS):
		c[i] = []
		c[i].resize(NUM_COLS)
		for j in range(NUM_COLS):
			c[i][j] = null

	return board

func addPiece(piece: Piece):
	return null

func drop():
	return null

# Colors has type Array[Array[Color]]
var colors: Array[Array]
