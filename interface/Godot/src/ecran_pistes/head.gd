extends TextureRect

const bounds:Vector2i = Vector2i(
	222,
	1145,
)

var mouse_in :=false
var dragging := false

func _ready():
	pass # Replace with function body.

func time_to_px(time:float):
	var fac = time / %Tracks.audio_length
	return int(fac * (bounds[1] - bounds[0]) + bounds[0])

func px_to_time(px):
	return (float(px - bounds[0])/ (bounds[1] - bounds[0])) * %Tracks.audio_length
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if dragging:
		
		position.x = clamp(
			get_viewport().get_mouse_position().x,
			bounds[0],
			bounds[1]
		)
		%Tracks.current_time = px_to_time(position.x)
	if not dragging and %Tracks.playing:
		
		position.x = time_to_px(%Tracks.current_time)
	



func _on_mouse_entered():
	mouse_in = true



func _on_mouse_exited():
	mouse_in = false

func _input(event):
	if mouse_in and event.is_action_pressed("click"):
		dragging = true
	if event.is_action_released("click"):
		dragging = false
		
		if %Tracks.playing:
			%Tracks.play()
