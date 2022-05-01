// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(float4 pos : A) : SV_TARGET
{
    return UnityObjectToViewPos(pos.xyz);
}
