// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float4 pos : COLOR0) : SV_TARGET
{
    float4 o = ComputeNonStereoScreenPos(pos);
#if defined(UNITY_SINGLE_PASS_STEREO)
    o.xy = TransformStereoScreenSpaceTex(o.xy, pos.w);
#endif
    return o;
}
