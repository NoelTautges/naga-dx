// SM: 4_1, 5_1

cbuffer Buffer_1
{
    float4x4 mat;
    float3 vec;
    float i;
};

cbuffer Buffer_2 : register(b2)
{
    float4 packed_vec : packoffset(c0);
    float packed_float_1 : packoffset(c1);
    float packed_float_2 : packoffset(c1.y);
};

float4 VSMain() : SV_Position
{
    return mat[0] + mat[1] + mat[2] + mat[3]
        + float4(vec, i)
        + packed_vec
        + float4(packed_float_1, packed_float_2, 0, 0);
}
