extends Node

@export var sample_A:AudioStream


@onready var playback

func start():
	$AudioStreamPlayer.play()
	playback = $AudioStreamPlayer.get_stream_playback()

func stop():
	$AudioStreamPlayer.stop()

func set_volume(v):
	$AudioStreamPlayer.volume_db = v

func play_note(note:int, length:float, velocity:int):
	
	var idx = playback.play_stream(
		sample_A,
		0,
		lerp(-80, -6, 1.0/128 * velocity),
		2**(note/12.0)
	)
	
	await get_tree().create_timer(length).timeout
	
	if playback.is_stream_playing(idx):
		playback.stop_stream(idx)
