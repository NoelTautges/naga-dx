// SM: 4_1, 5_0

#include "UnityCG.cginc"

float2 PSMain(float3 n : A) : SV_TARGET
{
    float kScale = 1.7777;
    float2 enc;
    enc = n.xy / (n.z+1);
    enc /= kScale;
    enc = enc*0.5+0.5;
    return enc;
}
