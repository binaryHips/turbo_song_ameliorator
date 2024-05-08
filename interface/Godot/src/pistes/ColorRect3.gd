extends ColorRect

var BLOCK = preload("res://src/generation_block/generation_block.tscn")

var mouse_in := false

var dragging := false
# Called when the node enters the scene tree for the first time.

var start_pos:Vector2i
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	
	if dragging:
		$highlight.scale.x = get_global_mouse_position().x - start_pos.x




func _on_mouse_entered():
	mouse_in = true


func _on_mouse_exited():
	mouse_in = false

func _input(event):
	if mouse_in && event.is_action_pressed("click"):
		start_block_creation()
	elif mouse_in && dragging && event.is_action_released("click"):
		finish_block_creation()
	elif not mouse_in && dragging && event.is_action_released("click"):
		abort_block_creation()

func start_block_creation():
	start_pos = get_global_mouse_position()
	$highlight.global_position.x = start_pos.x
	$highlight.show()
	dragging = true

func finish_block_creation():
	
	var end_pos = get_global_mouse_position()
	
	if end_pos.x < start_pos.x:
		
		var temp = start_pos.x
		start_pos.x = end_pos.x
		end_pos.x = temp
	
	if has_no_collision(start_pos.x, end_pos.x):
	
		var block:Control = BLOCK.instantiate()
		
		add_child(block)
		block.global_position.x = start_pos.x
		block.size.y = size.y
		block.size.x = end_pos.x - start_pos.x
	
	$highlight.hide()
	
	dragging = false


func has_no_collision(start:int, end:int):
	for i in range(1, get_child_count()): #0th is the highlight
		var c = get_child(i)
		if (
			(end > c.global_position.x && start < c.global_position.x + c.size.x)
			or (start < c.global_position.x + c.size.x && end > c.global_position.x ) 
			): return false
	return true

func abort_block_creation():
	
	dragging = false
	$highlight.hide()
