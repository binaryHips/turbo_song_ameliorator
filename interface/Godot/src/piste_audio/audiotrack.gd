extends Control

var can_drop_file := false

var stream:AudioStream
var stream_length := 0.0
func _ready():
	get_tree().root.files_dropped.connect(on_files_dropped)
	$Audio_text.text = "[center]No audio loaded!\n(drop an ogg vorbis or mp3 file to start)"

func on_files_dropped(files:PackedStringArray):
	var type = files[0].get_extension()
	if not type in ["ogg", "mp3", "wav", "WAVE"]:
		
		print("can't load that type of file")
		return
	
	match type:
		"ogg":
			stream = load_ogg(files[0])
		"mp3":
			stream = load_mp3(files[0])
		_:
			print("Wav files are not yet supported, sorry!")
			return
	
	$Audio_text.text = "[center]File loaded\n'" + files[0].split('/')[-1] + "'"
	$AudioStreamPlayer.stream = stream
	print(files[0])
	#$AudioStreamPlayer.play()
	


func load_ogg(path):
	return AudioStreamOggVorbis.load_from_file(path)
	
func load_mp3(path):
	var file = FileAccess.open(path, FileAccess.READ)
	var sound = AudioStreamMP3.new()
	sound.data = file.get_buffer(file.get_length())
	return sound


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_color_rect_3_mouse_entered():
	can_drop_file = true


func _on_color_rect_3_mouse_exited():
	can_drop_file = false


func _on_v_slider_value_changed(value):
	$AudioStreamPlayer.volume_db = value

func seek(time):
	$AudioStreamPlayer.seek(time)
func play():
	$AudioStreamPlayer.play()

func stop():
	$AudioStreamPlayer.stop()
