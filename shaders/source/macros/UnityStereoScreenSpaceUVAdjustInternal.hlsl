// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float4 uv : A, float4 scaleAndOffset : B) : SV_TARGET
{
    return float4(UnityStereoScreenSpaceUVAdjustInternal(uv.xy, scaleAndOffset), UnityStereoScreenSpaceUVAdjustInternal(uv.zw, scaleAndOffset));
}
