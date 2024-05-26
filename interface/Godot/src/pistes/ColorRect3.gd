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
	elif dragging && event.is_action_released("click"):
		finish_block_creation()

func start_block_creation():
	start_pos = get_global_mouse_position()
	$highlight.global_position.x = start_pos.x
	$highlight.show()
	dragging = true

func finish_block_creation():
	
	var end_pos := get_global_mouse_position()
	
	if end_pos.x < start_pos.x:
		
		var temp = start_pos.x
		start_pos.x = end_pos.x
		end_pos.x = temp
	
	start_pos.x = max(start_pos.x, global_position.x)
	end_pos.x = min(end_pos.x, global_position.x + size.x)
	if end_pos.x - start_pos.x > 10:
		var cols = collision(start_pos.x, end_pos.x)


			
		var min_start := start_pos.x
		var max_end := end_pos.x

		for c in cols:
			min_start = min(get_parent().generation_calls[c]["start_px"], min_start)
			max_end = max( get_parent().generation_calls[c]["end_px"], max_end)
			
			get_parent().generation_calls.erase(c)
			c.queue_free()

		var block:Control = BLOCK.instantiate()

		add_child(block)
		block.global_position.x = min_start
		block.size.y = size.y
		block.size.x = max_end - min_start
	$highlight.hide()
	
	dragging = false


func collision(start:int, end:int):
	var col = []
	for i in range(1, get_child_count()): #0th is the highlight
		var c = get_child(i)
		if (
			(end > c.global_position.x && start < c.global_position.x + c.size.x)
			or (start < c.global_position.x + c.size.x && end > c.global_position.x ) 
			): col.append(c)
	return col

func abort_block_creation():
	
	dragging = false
	$highlight.hide()
