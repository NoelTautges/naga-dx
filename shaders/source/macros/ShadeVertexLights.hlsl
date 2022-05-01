// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(float4 vertex : COLOR0, float3 normal : COLOR1) : SV_TARGET
{
    return ShadeVertexLightsFull (vertex, normal, 4, false);
}
