extends PopupPanel

@onready var container = $Control/VBoxContainer
var tab = ['A','A#', 'B', 'C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#']
# Called when the node enters the scene tree for the first time.
func _ready():
	for i in 12 :
		var button = Button.new()
		button.text = tab[i]
		button.pressed.connect(scale_changed)
		button.toggle_mode=true
		button.custom_minimum_size=Vector2i(100,30)
		button.add_theme_font_size_override("font_size", 12)
		container.add_child(button)
		
		$Control/OptionButton.add_item(tab[i])
	


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func scale_changed():
	for i in container.get_child_count():
		var b:Button = container.get_child(i)
		get_parent().notes_in_scale[i]= b.button_pressed
		
	get_parent().scale_changed()


func _on_option_button_item_selected(index):
	get_parent().root = index
	get_parent().scale_changed()
