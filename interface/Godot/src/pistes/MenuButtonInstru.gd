extends MenuButton


# Called when the node enters the scene tree for the first time.
func _ready():
	for i in GenerationData.instruments:
		get_popup().add_item(i)
	
	get_popup().index_pressed.connect(_index_pressed)
	_index_pressed(0)

func _index_pressed(index:int):
	var instrument := get_popup().get_item_text(index)
	text = instrument
	get_parent().get_parent().instrument = instrument
	
