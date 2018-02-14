#version 330

in vec2 position;

out vec2 frag_position;

uniform float left;
uniform float right;
uniform float top;
uniform float bottom;

void main() {
	gl_Position = vec4(position, 0.0, 1.0);

	frag_position = (position * vec2(0.5) + vec2(0.5)) * vec2(right - left, top - bottom) + vec2(left, bottom);
}
