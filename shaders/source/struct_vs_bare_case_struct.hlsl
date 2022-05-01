// SM: 4_1, 5_0

struct vs_input {
    float4 position : SV_Position;
};

float4 VSMain(const vs_input input) : SV_Position
{
    return input.position;
}
