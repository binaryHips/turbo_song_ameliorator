extends VBoxContainer
const PISTE = preload("res://src/pistes/piste.tscn")

@onready var button = $Button
@onready var counter = get_parent().get_node("counter")
# Called when the node enters the scene tree for the first time.
func _ready():
	$audio_track.file_changed.connect(file_changed)
	$audio_track.track_status_changed.connect(redo_track_status)
	
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if playing:
		current_time += delta
		
		if current_time > audio_length:
			stop()



func _on_button_pressed():
	var nouvelle_piste = PISTE.instantiate()
	nouvelle_piste.track_status_changed.connect(redo_track_status)
	add_child(nouvelle_piste)
	remove_child(button)
	add_child(button)


var audio_length := 60.0
var playing := false
var current_time := 0.0:
	set(v):
		current_time = v
		counter.text = str(int(current_time) / 60) + ":" + str(int(current_time) % 60)
		

func set_playback_start(time:float):
	current_time = time
	#for i in get_child_count()-1: #-1 for the button
		#var c = get_child(i)
		#c.seek(time)
		
		

func file_changed():
	audio_length = $audio_track.stream_length
	current_time = %head.px_to_time(%head.position.x)

func play():
	playing = true
	for i in get_child_count()-1: #-1 for the button
		var c = get_child(i)
		c.play(current_time)
		
func stop():
	playing = false
	for i in get_child_count()-1:
		var c = get_child(i)
		c.stop()

var solo_mode := false
func redo_track_status():
	solo_mode = false
	for i in get_child_count()-1:
		var c = get_child(i)
		solo_mode = solo_mode or c.solo
	
	for i in get_child_count()-1:
		var c = get_child(i)
		
		if solo_mode:
			c.deactivated = not c.solo
		else:
			c.deactivated = c.muted


func _on_play_toggled(toggled_on):
	if toggled_on:
		play()
	else:
		stop()

