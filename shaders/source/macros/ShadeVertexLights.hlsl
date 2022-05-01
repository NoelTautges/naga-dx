// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(float4 vertex : A, float3 normal : B) : SV_TARGET
{
    return ShadeVertexLightsFull (vertex, normal, 4, false);
}
