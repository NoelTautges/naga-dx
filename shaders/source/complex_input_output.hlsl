// SM: 4_1, 5_1

struct vs_input {
    float4 position : SV_Position;
    float4 normal : NORMAL;
};

struct vs_output {
    float4 position : SV_Position;
    float4 color : COLOR;
};

vs_output VSMain(const vs_input input)
{
    vs_output output;
    output.position = input.position;
    output.color = input.normal;
    return output;
}
