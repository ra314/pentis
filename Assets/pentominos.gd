class_name Pentominos

static func parse_piece(raw_piece: String) -> Array[Vector2i]:
	var ret := []
	var y := 0
	var x := 0
	for line in raw_piece.split('\n'):
		for char in line:
			if char == "0":
				ret.append(Vector2i(x, y))
			x += 1
		y += 1
	return ret

const PIECES := {
	GLOBALS.PIECE.F: {
		GLOBALS.ROTATION.ROT0: F0,
		GLOBALS.ROTATION.ROT90: F90,
		GLOBALS.ROTATION.ROT180: F180,
		GLOBALS.ROTATION.ROT270: F270
	},
	GLOBALS.PIECE.I: {
		GLOBALS.ROTATION.ROT0: I0,
		GLOBALS.ROTATION.ROT90: I90,
		GLOBALS.ROTATION.ROT180: I180,
		GLOBALS.ROTATION.ROT270: I270
	},
	GLOBALS.PIECE.L: {
		GLOBALS.ROTATION.ROT0: L0,
		GLOBALS.ROTATION.ROT90: L90,
		GLOBALS.ROTATION.ROT180: L180,
		GLOBALS.ROTATION.ROT270: L270
	}
}

const F0 :=  "
.00
00.
.0.
"
const F90 :=  "
.0.
000
..0
"
const F180 :=  "
.0.
.00
00.
"
const F270 :=  "
0..
000
.0.
"

const I0 := "
.0.
.0.
.0.
.0.
.0.
"
const I90 := "
.....
00000
.....
"
const I180 := "
.0.
.0.
.0.
.0.
.0.
"
const I270 := "
.....
00000
.....
"

const L0 := "
.0..
.0..
.0..
.00.
"
const L90 := "
....
0000
0...
....
"
const L180 := "
.00.
..0.
..0.
..0.
"
const L270 := "
....
...0
0000
....
"
