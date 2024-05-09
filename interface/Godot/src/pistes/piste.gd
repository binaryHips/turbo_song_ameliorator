extends Control

var volume:float = 0.0

var algorithm:String = ""
var instrument:String = ""

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
			$AudioStreamPlayer.volume_db = -80
			$ColorRect2.self_modulate = Color.DARK_SLATE_GRAY
		else:
			$AudioStreamPlayer.volume_db = volume
			$ColorRect2.self_modulate = Color.WHITE

# Called when the node enters the scene tree for the first time.
func _ready():
	$AudioStreamPlayer.volume_db = volume


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass



func play(time:float):
	pass

func stop():
	pass


func _on_v_slider_value_changed(value):
	volume = value
	
	if not deactivated:
		$AudioStreamPlayer.volume_db = volume
