extends MenuButton


# Called when the node enters the scene tree for the first time.
func _ready():
	get_popup().add_item("Algo 1")
	get_popup().add_item("Algo 2")
	get_popup().add_item("Algo 3")
	get_popup().add_item("Algo 4")
	get_popup().index_pressed.connect(_index_pressed)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func _index_pressed(index:int):
	var algo_name := get_popup().get_item_text(index)
	print(algo_name)
	text = algo_name
