extends Button


# Called when the node enters the scene tree for the first time.
func _ready():
	text = "Unsolo"


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass



func _on_pressed():
	if text == "Solo" :
		text = "UnSolo"
		get_parent().get_parent().solo = true
	else :
		text = "Solo"
		get_parent().get_parent().solo = false



