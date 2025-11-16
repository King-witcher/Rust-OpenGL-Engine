#version 460 core

layout (location = 0) in vec3 in_position;
layout (location = 1) in vec3 in_color;
layout (location = 2) in vec2 in_uv;

layout (location = 0) out vec3 out_color;
layout (location = 1) out vec2 out_uv;

layout (location = 0) uniform mat4 model;
layout (location = 1) uniform mat4 camera;

vec3 positions[] = vec3[](
    vec3(-1.0, -1.0, 0.0),
    vec3(1.0, -1.0, 0.0),
    vec3(1.0, 1.0, 0.0)
);

void main() {
    gl_Position = camera * model * vec4(in_position, 1.0);
    out_color = in_color;
    out_uv = in_uv;
}
