#version 330 core

layout (location = 0) in vec3 position;
layout (location = 1) in vec2 uv_coords;
layout (location = 3) in mat4 transform;

uniform mat4 vp;

out vec2 tex_coords;

void main() {
    gl_Position = vp * transform * vec4(position, 1.0);
    tex_coords = uv_coords;
}
