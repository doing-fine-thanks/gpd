extends GPD


# Called when the node enters the scene tree for the first time.
func _ready():
	self.audio_generator = $AudioStreamPlayer.get_stream_playback()
	self.play()
