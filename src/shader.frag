#version 330 core

in vec2 tex_coords;
out vec4 colour;

uniform sampler2D tex;

void main() {
    colour = texture(tex, tex_coords);
    if (colour.a == 0.0) {
        discard;
    }
}
