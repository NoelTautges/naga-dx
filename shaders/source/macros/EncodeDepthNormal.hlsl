// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float depth : COLOR0, float3 normal : COLOR1) : SV_TARGET
{
    float4 enc;
    enc.xy = EncodeViewNormalStereo (normal);
    enc.zw = EncodeFloatRG (depth);
    return enc;
}
