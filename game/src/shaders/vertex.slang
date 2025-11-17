// #version 460 core

// layout (location = 0) in vec3 in_position;
// layout (location = 1) in vec3 in_color;
// layout (location = 2) in vec2 in_uv;

// layout (location = 0) out vec3 out_color;
// layout (location = 1) out vec2 out_uv;

// layout (location = 0) uniform mat4 model;
// layout (location = 1) uniform mat4 camera;

// void main() {
//     gl_Position = camera * model * vec4(in_position, 1.0);
//     out_color = in_color;
//     out_uv = in_uv;
// }

struct VSInput
{
    float3 position : POSITION;
    float3 color : COLOR;
    float2 uv : TEXCOORD;
}

struct VSOutput
{
    float4 position : SV_Position;
    float3 color : COLOR;
    float2 uv : TEXCOORD;
}

struct UniformBuffer
{
    float4x4 model;
    float4x4 camera;
}

ConstantBuffer<UniformBuffer> ubo;

[shader("vertex")]
VSOutput main(VSInput input)
{
    VSOutput output;
    output.position = mul(ubo.camera, mul(ubo.model, float4(input.position, 1.0)));
    output.color = input.color;
    output.uv = input.uv;
    return output;
}
