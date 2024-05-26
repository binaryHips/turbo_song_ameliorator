extends VBoxContainer
const PISTE = preload("res://src/pistes/piste.tscn")

@onready var button = $Button
@onready var counter = get_parent().get_parent().get_node("counter")
# Called when the node enters the scene tree for the first time.
func _ready():
	$audio_track.file_changed.connect(file_changed)
	$audio_track.track_status_changed.connect(redo_track_status)
	GenerationData.tracks_manager = self
	
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
		counter.text = format_time(current_time)
		

func format_time(t):
	var min = int(current_time / 60)
	var sec = (int(current_time) % 60)
	var ms = int(fposmod(current_time,1) * 1000)
	
	
	
	return (
		("0" + str(min) if min < 10 else str(min) ) + ":" +
		("0" + str(sec)if sec < 10 else str(sec) ) + "::" +
		("00" + str(ms) if ms < 10 else "0" + str(ms)if ms < 100 else str(ms))
		
	)

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
	for i in get_child_count():
		var c = get_child(i)
		
		if not c.has_method("play"): continue #duck typing
		
		c.play(current_time)
		
func stop():
	playing = false
	for i in get_child_count():
		var c = get_child(i)
		
		if not c.has_method("play"): continue
		
		c.stop()

var solo_mode := false
func redo_track_status():
	solo_mode = false
	for i in get_child_count()-1:
		var c = get_child(i)
		
		if not c.has_method("play"): continue
		
		solo_mode = solo_mode or c.solo
	
	for i in get_child_count()-1:
		var c = get_child(i)
		
		if not c.has_method("play"): continue
		
		if solo_mode:
			c.deactivated = not c.solo
		else:
			c.deactivated = c.muted


func _on_play_toggled(toggled_on):
	if toggled_on:
		play()
	else:
		stop()

func _input(event):
	if event.is_action_pressed("play_stop"):
		if playing:
			stop()
		else:
			play()

func px_to_time(px):
	return %head.px_to_time(px) #TODO refactor this here.

func time_to_px(time):
	return %head.time_to_px(time) #TODO refactor this here.
