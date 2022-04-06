struct vs_input {
    float4 position : SV_Position;
    float4 color : COLOR;
};

float4 VSMain(const vs_input input) : SV_Position
{
    float4 temp = input.position * input.color;
    float4 temp2 = temp * input.color;
    return temp2 * input.color;
}
