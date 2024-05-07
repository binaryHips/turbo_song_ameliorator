extends TextureRect


const bounds:Vector2i = Vector2i(
	222,
	1145,
)

var mouse_in :=false
var dragging := false

func _ready():
	pass # Replace with function body.
	

func _process(delta):
	if dragging:
		
		position.x = clamp(
			get_viewport().get_mouse_position().x,
			bounds[0],
			bounds[1]
		)
	



func _on_mouse_entered():
	mouse_in = true



func _on_mouse_exited():
	mouse_in = false

func _input(event):
	if mouse_in and event.is_action_pressed("click"):
		dragging = true
	if event.is_action_released("click"):
		dragging = false
		
