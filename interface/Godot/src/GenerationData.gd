extends Node

var tracks_manager:Node

var algorithms = ["Markov", "Dice", "Perc"]

var instruments = ["Saw", "Guitar", "Perc kit"]


#keys are ref of generation_blocks
#values are dicts containing algo, start time etc
var tracks = []
#scale objects in the UI
var scales = []

var anaData:MusicAnalysisData
#MusicAnalysisData.create(
#	120,
#	[0, 2, 3, 5, 7, 8, 10] as Array[int],
#	0,
#	5.0
#	)

func _ready():
	pass
	
func _process(delta):
	pass

func generate_all():
	if anaData == null: return
	
	for track in tracks:
		track.notes.clear()
		
		if track.algorithm == "": return
		
		var generator = MusicGenerator.create(anaData, track.algorithm)
		
		for k in track.generation_calls.keys():
			k.set_generated(false)
			var block_info = track.generation_calls[k]
			generator.generate(
				tracks_manager.px_to_time(block_info.start_px),
				tracks_manager.px_to_time(block_info.end_px),
			)
			
			var notes = generator.get_notes()
			
			for n in len(notes):
				notes[n].start_time += tracks_manager.px_to_time(block_info.start_px)
				notes[n].end_time += tracks_manager.px_to_time(block_info.start_px)
			
			track.notes.append_array(notes)
			k.set_generated(true)
func analyse_file(path:String):
	anaData = MusicAnalysisData.create_from_file(path)
