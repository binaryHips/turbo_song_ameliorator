extends Control

@onready var AnaData = $Anadata

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
	move_anadata.call_deferred()

#hacky but works. Moves anadata under the track
func move_anadata():
	AnaData.reparent(get_parent())
	get_parent().remove_child(get_parent().button)
	get_parent().add_child(get_parent().button)


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
	
	if "/" in files[0]:
		
		$Audio_text.text = "[center]File loaded\n'" + files[0].split('/')[-1] + "'"
	else:
		$Audio_text.text = "[center]File loaded\n'" + files[0].split("\\")[-1] + "'"
	$AudioStreamPlayer.stream = stream
	stream_length = stream.get_length()
	file_changed.emit()
	print(files[0])
	#$AudioStreamPlayer.play()
	GenerationData.analyse_file(files[0])
	print("TIME BEFORE START ", GenerationData.anaData.get_time_before_start())
	#AnaData.get_node("Anadata2/Start_time").global_position.x = get_parent().time_to_px(
	#	GenerationData.anaData.get_time_before_start()
	#)
	AnaData.get_node("offset").text = "%.3f" % GenerationData.anaData.get_time_before_start()
	
	var scale = AnaData.get_node("Anadata2").SCALE.instantiate()
	AnaData.get_node("Anadata2").add_child(scale)
	scale.make_scale(GenerationData.anaData)
	scale.position.x = 0
	AnaData.get_node("bpm").text = str(GenerationData.anaData.get_bpm())
	$AudioStreamPlayer.volume_db = volume
	$AudioStreamPlayer.play()
	$AudioStreamPlayer.stop()
	
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

var anadata_tween:Tween
func _on_check_analysis_data_toggled(toggled_on):
	anadata_tween = get_tree().create_tween()
	if toggled_on:
		AnaData.visible = true
		anadata_tween.tween_property(AnaData, "custom_minimum_size", Vector2(100,65), 0.1)
		anadata_tween.set_ease(Tween.EASE_IN_OUT)
		anadata_tween.set_trans(Tween.TRANS_EXPO)
		anadata_tween.play()
	else:
		anadata_tween.tween_property(AnaData, "custom_minimum_size", Vector2(100,0), 0.1)
		anadata_tween.set_ease(Tween.EASE_IN_OUT)
		anadata_tween.set_trans(Tween.TRANS_EXPO)
		anadata_tween.play()
		await anadata_tween.finished
		AnaData.visible = false
