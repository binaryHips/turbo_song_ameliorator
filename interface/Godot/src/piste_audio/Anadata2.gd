extends ColorRect

const SCALE = preload("res://src/scale/scale.tscn")

@onready var bpm = get_parent().get_node("bpm")
@onready var offset = get_parent().get_node("offset")
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
		
		# Put that back in when multiple scales are supported
		#var scale = SCALE.instantiate()
		
		#scale.position.x = get_local_mouse_position().x
		#add_child(scale)
	


func _on_bpm_text_set():
	var v:String = bpm.text
	if v.is_valid_int():
	
		GenerationData.anaData = MusicAnalysisData.create(
			int(v),
			GenerationData.anaData.get_scale_notes(),
			GenerationData.anaData.get_scale_root(),
			GenerationData.anaData.get_time_before_start()
			
		)
		
		print("BPM ", GenerationData.anaData.get_bpm())


func _on_offset_changed():
	var v:String = offset.text
	if v.is_valid_int():
	
		GenerationData.anaData = MusicAnalysisData.create(
			GenerationData.anaData.get_bpm(),
			GenerationData.anaData.get_scale_notes(),
			GenerationData.anaData.get_scale_root(),
			float(v),
			
		)
		
		print("BPM ", GenerationData.anaData.get_bpm())
