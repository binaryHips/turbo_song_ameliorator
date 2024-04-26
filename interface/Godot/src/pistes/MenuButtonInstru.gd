extends MenuButton


# Called when the node enters the scene tree for the first time.
func _ready():
	get_popup().add_item("Cordes")
	get_popup().add_item("Vents")
	get_popup().add_item("Percussions")
	get_popup().add_item("Sons Funs")
	get_popup().index_pressed.connect(_index_pressed)

func _index_pressed(index:int):
	var type_instru := get_popup().get_item_text(index)
	print(type_instru)
	
	get_node(type_instru).show()
	
	
