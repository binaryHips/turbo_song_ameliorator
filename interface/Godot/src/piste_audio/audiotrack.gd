extends Control

var can_drop_file := false

var stream:AudioStream
var stream_length := 0.0

var volume:float = -6.0

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
			
func _ready():
	get_tree().root.files_dropped.connect(on_files_dropped)
	
	$Audio_text.text = "[center]No audio loaded!\n(drop an ogg vorbis or mp3 file to start)"
	
	$AudioStreamPlayer.volume_db = volume





signal file_changed

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
	stream_length = stream.get_length()
	file_changed.emit()
	print(files[0])
	#$AudioStreamPlayer.play()
	get_waveform_samples()

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
	volume = value
	
	if not deactivated:
		$AudioStreamPlayer.volume_db = volume


func play(time):
	$AudioStreamPlayer.play(time)

func stop():
	$AudioStreamPlayer.stop()


func get_waveform_samples():
	const N = 350
	var samples = []
	samples.resize(N)
	samples.fill(0)
	
	var spectrum:AudioEffectSpectrumAnalyzerInstance = AudioServer.get_bus_effect_instance(1, 0)
	$AudioStreamPlayer2.stream = stream;
	$AudioStreamPlayer2.bus = AudioServer.get_bus_name(1)
	$ColorRect3.material.set("shader_parameter/n", 0)
	##
	for i in range(N):
		$AudioStreamPlayer2.play(stream_length/N * i)
		
		samples[i] = (
			spectrum.get_magnitude_for_frequency_range(0, 20000)[1] #just for funsies
			)
		$ColorRect3.material.set("shader_parameter/samples", samples)
		$ColorRect3.material.set("shader_parameter/n", i+1)
		await get_tree().create_timer(0.01).timeout
	##
	$AudioStreamPlayer2.stop()


func _on_check_analysis_data_toggled(toggled_on):
	$Anadata.visible = toggled_on
	custom_minimum_size.y = 115 if not toggled_on else 115 + 65
