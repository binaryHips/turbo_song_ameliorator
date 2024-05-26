extends Node

@export var sample_A:AudioStream
@export var multisamples := false
@export var multisamples_list: Array[AudioStream] = []

@onready var playback:AudioStreamPlaybackPolyphonic

func start():
	$AudioStreamPlayer.play()
	playback = $AudioStreamPlayer.get_stream_playback()

func stop():
	$AudioStreamPlayer.stop()

func set_volume(v):
	$AudioStreamPlayer.volume_db = v

func play_note(note:int, length:float, velocity:int):
	var idx:int
	if multisamples:
			if len(multisamples_list) <= note: return
			idx = playback.play_stream(
			multisamples_list[note],
			0,
			lerp(-80, -6, 1.0/128 * velocity),
		)
	
	else:
		
		idx = playback.play_stream(
			sample_A,
			0,
			lerp(-80, -6, 1.0/128 * velocity),
			2**(note/12.0)
		)
	
	await get_tree().create_timer(length).timeout
	
	if playback.is_stream_playing(idx):
		playback.stop_stream(idx)
