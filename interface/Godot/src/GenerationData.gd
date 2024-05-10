extends Node


var algorithms = ["Markov", "Alt. Markov"]

var instruments = ["Sine", "Saw"]


#keys are ref of generation_blocks
#values are dicts containing algo, start time etc
var generation_calls = {}

#scale objects in the UI
var scales = []

func _ready():
	pass
	
	
