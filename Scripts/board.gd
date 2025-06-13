extends Node2D

class_name Board

var current_piece: Piece
# Colors has type Array[Array[Color]]
var colors: Array[Array]

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

func addPiece(piece: Piece) -> void:
	current_piece = piece

func clearPiece() -> void:
	for vec in current_piece.getVectors():
		colors[vec.x][vec.y] = null

func drawPiece() -> void:
	for vec in current_piece.getVectors():
		colors[vec.x][vec.y] = current_piece.color

func stop_drop() -> void:
	current_piece.undrop()
	current_piece = null

# Check if there are any collisions below.
func drop() -> void:
	current_piece.drop()
	for vec in current_piece.get_vectors():
		if vec.y >= GLOBALS.BOARD_ROWS:
			return stop_drop()
		if colors[vec.y][vec.x] != Color.BLACK:
			return stop_drop()
