// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(in float4 v : COLOR0) : SV_TARGET
{
    float3 objSpaceCameraPos = mul(unity_WorldToObject, float4(_WorldSpaceCameraPos.xyz, 1)).xyz;
    return objSpaceCameraPos - v.xyz;
}
