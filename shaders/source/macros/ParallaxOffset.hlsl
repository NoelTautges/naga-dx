// SM: 4_1, 5_0

#include "UnityCG.cginc"

float2 PSMain(half h : COLOR0, half height : COLOR1, half3 viewDir : COLOR2) : SV_TARGET
{
    h = h * height - height/2.0;
    float3 v = normalize(viewDir);
    v.z += 0.42;
    return h * (v.xy / v.z);
}
