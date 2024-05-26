extends Control




# Called when the node enters the scene tree for the first time.
func _ready():
	set_self.call_deferred()

func set_self():
	get_parent().get_parent().generation_calls[self] = {
		"start_px": global_position.x,
		"end_px": global_position.x + size.x,
		
		}
# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func set_generated(v):
	$generated.visible = v
