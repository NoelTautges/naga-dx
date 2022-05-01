// SM: 4_1, 5_0

struct vs_input {
    float3 position : SV_Position;
    float psize : PSIZE;
    uint blend : BLENDINDICES;
};

float4 VSMain(const vs_input input) : SV_Position
{
    return float4(input.position.xy, input.position.z + input.psize, input.blend);
}
