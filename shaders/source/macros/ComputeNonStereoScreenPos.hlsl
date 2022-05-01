// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float4 pos : A) : SV_TARGET
{
    float4 o = pos * 0.5f;
    o.xy = float2(o.x, o.y*_ProjectionParams.x) + o.w;
    o.zw = pos.zw;
    return o;
}
