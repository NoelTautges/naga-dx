// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float3 vertex : COLOR0, float3 normal : COLOR1) : SV_TARGET
{
    return UnityClipSpaceShadowCasterPos(float4(vertex, 1), normal);
}
