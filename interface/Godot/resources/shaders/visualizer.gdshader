shader_type canvas_item;

const int SAMPLES_N = 350; //Must be reciprocated in the gdscript caller!

uniform float[SAMPLES_N] samples;
uniform int n;

void vertex() {
	// Called for every vertex the material is visible on.

	
}

void fragment() {
	
	
	int i = int(UV.x * float(SAMPLES_N));
	float h = 0.0;

	
	COLOR = vec4(0.5, 0.1, 0.1, 1.0);
	
	float dist = abs(UV.y-0.5);
	float max_dist = 0.01;
	max_dist = (samples[i]*5.0 + 0.01) * min(1.0, (distance(float(i), float(n))/20.0));
	max_dist *= 0.5;
	max_dist = max(max_dist, 0.01);

	if (dist < max_dist){
		
		COLOR = vec4(1.0, 1.0-abs(UV.y-0.5) - 0.8, 1.0-abs(UV.y-0.5) - 0.8, 1.0);
	}
	
}

	
//void light() {
	// Called for every pixel for every light affecting the CanvasItem.
	// Uncomment to replace the default light processing function with this one.
//}
