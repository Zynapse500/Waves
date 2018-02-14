#version 330

out vec4 out_color;

in vec2 frag_position;

uniform vec2 center0;
uniform vec2 center1;
uniform float time;

uniform float color;

uniform float wavelength;
uniform float frequency;


#define PI 3.1415926535897932384626433832795

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}


float amplitude(vec2 center) {
    float velocity = frequency * wavelength;
    return sin(distance(frag_position, center) / wavelength * 2.0 * PI - time * velocity / (2.0 * PI));
}


void main() {
    float value = 0.0;


    value += color * amplitude(center0);
    value += (1.0 - color) * amplitude(center1);


    float color = 1.0 - (value + 1.0) / 2.0;
	out_color = vec4(vec3(value), 1.0);

	out_color = pow(out_color, vec4(2.2,2.2,2.2,1.0));
}
