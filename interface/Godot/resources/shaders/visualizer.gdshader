shader_type canvas_item;

const int SAMPLES_N = 500; //Must be reciprocated in the gdscript caller!

uniform float[SAMPLES_N] samples;


void vertex() {
	// Called for every vertex the material is visible on.
	int i = int(UV.x * 500.0);
	
	float h = samples[i];
	
}

void fragment() {
	// Called for every pixel the material is visible on.
}

//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
