extends TextureRect

var notes_in_scale = [false,false,false,false,false,false,false,false,false,false,false,false]
var root:int = 0
var mouse_in := false


var lock = true #prevent async problems frominstancing
func _ready():
	GenerationData.scales.append(self)


func _process(delta):
	pass

func scale_changed():
	$notes_text.text = ""
	for i in 12:
		if notes_in_scale[i]:
			$notes_text.text += $PopupPanel.tab[i] + ", "
			
	
	print($notes_text.text)
	$notes_text.text.trim_suffix(",")
	
	#change if multiple scales
	var s:Array[int] = []
	for i in 12:
		if notes_in_scale[i]: s.append(i)
	
	if not lock:
		GenerationData.anaData = MusicAnalysisData.create(
			GenerationData.anaData.get_bpm(),
			s,
			root,
			GenerationData.anaData.get_time_before_start()
		)


func make_scale(data:MusicAnalysisData):
	
	var notes = data.get_scale_notes()
	root = data.get_scale_root()
	
	$PopupPanel.get_node("Control/CanvasLayer/OptionButton").selected = root
	print("root:", root)
	
	for i in 12:
		notes_in_scale[i] = i in notes
	
	for i in 12:
		if notes_in_scale[i]:
			$notes_text.text += $PopupPanel.tab[i] + ", "
			
	for i in $PopupPanel.container.get_child_count():
		var b:Button = $PopupPanel.container.get_child(i)
		b.button_pressed = notes_in_scale[i]
	lock = false

func _on_mouse_entered():
	mouse_in = true


func _on_mouse_exited():
	mouse_in = false

func _input(event):
	if mouse_in and event.is_action_pressed("click"):
		$PopupPanel.position = global_position + Vector2(50, -50)
		$PopupPanel.show()
	#elif mouse_in and event.is_action_pressed("alt click"):
	#	queue_free()

func _exit_tree():
	GenerationData.scales.erase(self)
