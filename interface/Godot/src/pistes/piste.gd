extends Control

var volume:float = 0.0

var algorithm:String = ""
			
var instrument:String = "Saw"

var generation_calls := {}
var notes:Array[MusicNote] = []


signal track_status_changed #soloing has side effects
var muted := false:
	set(v):
		muted = v
		track_status_changed.emit()
		
var solo := false:
	set(v):
		solo = v
		track_status_changed.emit()
		
var deactivated:= false:
	set(v):
		deactivated = v
		if deactivated:
			for k in $instruments.get_children():
				k.set_volume(-80)
			$ColorRect2.self_modulate = Color.DARK_SLATE_GRAY
		else:
			for k in $instruments.get_children():
				k.set_volume(volume)
			$ColorRect2.self_modulate = Color.WHITE

# Called when the node enters the scene tree for the first time.
func _ready():
	$instruments.get_node(instrument).set_volume(volume)
	GenerationData.tracks.append(self)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	
	if get_parent().playing:
		#not ideal but should work for small projects
		for n in notes:
			if abs(n.start_time - (get_parent().current_time)) < delta/2.0: #delta so we don't repeat notes
				
				print("NOTE ", n.start_time, ' ', get_parent().current_time)
				$instruments.get_node(instrument).play_note(n.note, n.end_time - n.start_time, n.velocity)

func play(time:float):
	$instruments.get_node(instrument).start()

func stop():
	$instruments.get_node(instrument).stop()


func _on_v_slider_value_changed(value):
	volume = value
	
	if not deactivated:
		for k in $instruments.get_children():
			k.set_volume(volume)
