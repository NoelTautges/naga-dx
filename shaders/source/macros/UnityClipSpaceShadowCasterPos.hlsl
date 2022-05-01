// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float3 vertex : A, float3 normal : B) : SV_TARGET
{
    return UnityClipSpaceShadowCasterPos(float4(vertex, 1), normal);
}
