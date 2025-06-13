class_name Numpy

static func add(slice: Array[Vector2i], vec: Vector2i) -> Array[Vector2i]:
	return slice.map(func(x: Vector2i): return x+vec)
