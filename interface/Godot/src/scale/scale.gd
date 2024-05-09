extends TextureRect

var notes_in_scale = [false,false,false,false,false,false,false,false,false,false,false,false]

var mouse_in := false
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


func _on_mouse_entered():
	mouse_in = true


func _on_mouse_exited():
	mouse_in = false

func _input(event):
	if mouse_in and event.is_action_pressed("click"):
		$PopupPanel.position = global_position + Vector2(50, -50)
		$PopupPanel.show()
	elif mouse_in and event.is_action_pressed("alt click"):
		queue_free()

func _exit_tree():
	GenerationData.scales.erase(self)
