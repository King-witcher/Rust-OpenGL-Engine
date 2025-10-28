#version 460

layout (location = 0) in vec3 inColor;
layout (location = 1) in vec2 inTexCoord;

layout (location = 0) out vec4 fragColor;

uniform sampler2D texture1;

void main()
{
  fragColor = vec4(inColor, 1.0);
}
