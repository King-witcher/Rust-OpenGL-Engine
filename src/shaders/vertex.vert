#version 460 core
layout (location = 0) in vec2 inPosition;
layout (location = 1) in vec3 inColor;
layout (location = 2) in vec2 inUV;

layout (location = 0) out vec3 outColor;
layout (location = 1) out vec2 outUV;

void main() {
    gl_Position = vec4(inPosition.x * 3 / 4, inPosition.y, 0.0, 1.0);
    outColor = inColor;
    outUV = inUV;
}
