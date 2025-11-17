struct FSInput
{
    float3 color : COLOR;
    float2 uv : TEXCOORD;
}

struct FSOutput
{
    float4 color : SV_Target;
}

Texture2D texture1;
SamplerState sampler1;

[shader("fragment")]
FSOutput main(FSInput input)
{
    FSOutput output;
    output.color = texture1.Sample(sampler1, input.uv);
    return output;
}
