// SM: 4_1, 5_0

#include "UnityCG.cginc"

float2 PSMain(float4x4 mat : A, float2 inUV : B) : SV_TARGET
{
    float4 temp = float4 (inUV.x, inUV.y, 0, 0);
    temp = mul (mat, temp);
    return temp.xy;
}
