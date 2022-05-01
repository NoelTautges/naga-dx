// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(in float4 localPos : COLOR0) : SV_TARGET
{
    float3 worldPos = mul(unity_ObjectToWorld, localPos).xyz;
    return UnityWorldSpaceViewDir(worldPos);
}
