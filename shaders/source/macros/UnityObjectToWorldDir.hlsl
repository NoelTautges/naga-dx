// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(in float3 dir : COLOR0) : SV_TARGET
{
    return normalize(mul((float3x3)unity_ObjectToWorld, dir));
}
