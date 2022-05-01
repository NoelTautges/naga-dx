// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(in float3 pos : COLOR0) : SV_TARGET
{
    return mul(UNITY_MATRIX_P, float4(pos, 1.0));
}
