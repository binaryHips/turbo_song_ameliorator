extends PopupPanel

@onready var container = $Control/VBoxContainer

var notes_in_scale = [false,false,false,false,false,false,false,false,false,false,false,false]
# Called when the node enters the scene tree for the first time.
func _ready():
	var tab = ['A','A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#']
	for i in 12 :
		var button = Button.new()
		button.text = tab[i]
		button.pressed.connect(scale_changed)
		button.toggle_mode=true
		button.custom_minimum_size=Vector2i(200,45)
		container.add_child(button)
	


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func scale_changed():
	for i in container.get_child_count():
		var b = container.get_child(i)
		notes_in_scale[i]= b.button_pressed
	print(notes_in_scale)
		
