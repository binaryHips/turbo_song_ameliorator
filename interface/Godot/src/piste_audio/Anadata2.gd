extends ColorRect

const SCALE = preload("res://src/scale/scale.tscn")

var mouse_in := false
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass



func _on_mouse_entered():
	mouse_in = true




func _on_mouse_exited():
	mouse_in = false


func _input(event):
	if mouse_in && event.is_action_pressed("click"):
		
		#quick check if we're on another control. Not really expensive
		for c in get_children():
			if c.mouse_in: return
		
		var scale = SCALE.instantiate()
		
		scale.position.x = get_local_mouse_position().x
		add_child(scale)
	
