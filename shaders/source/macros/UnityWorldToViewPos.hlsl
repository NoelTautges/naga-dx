// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(in float3 pos : COLOR0) : SV_TARGET
{
    return mul(UNITY_MATRIX_V, float4(pos, 1.0)).xyz;
}
