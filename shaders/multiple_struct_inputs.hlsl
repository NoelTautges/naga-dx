struct vs_input_1 {
    float3 position : SV_Position;
};

struct vs_input_2 {
    float psize : PSIZE;
};

float4 VSMain(const vs_input_1 input_1, const vs_input_2 input_2) : SV_Position
{
    return float4(input_1.position, input_2.psize);
}
