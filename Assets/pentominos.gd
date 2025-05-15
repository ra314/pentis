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

const F :=  "
.OO
OO.
.O.
"

const I := "
.0.
.0.
.0.
.0.
.0.
"

const L := "
0..
0..
0..
00.
"
