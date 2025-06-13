extends Node2D

class_name Board

var current_piece: Piece

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
	current_piece = piece

func clearPiece():
	for vec in range(current_piece.getVectors()):
		board.colors[vec.x][vec.y] = null

func drawPiece():
	for vec in range(current_piece.getVectors()):
		board.colors[vec.x][vec.y] = current_piece.color

func drop():
	# Check if there are any collisions below
	
	return 

# Colors has type Array[Array[Color]]
var colors: Array[Array]
