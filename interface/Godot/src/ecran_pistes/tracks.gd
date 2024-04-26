extends VBoxContainer
const PISTE = preload("res://src/pistes/piste.tscn")

@onready var button = $Button

# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass



func _on_button_pressed():
	var nouvelle_piste = PISTE.instantiate()
	add_child(nouvelle_piste)
	remove_child(button)
	add_child(button)
