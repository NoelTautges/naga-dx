// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float depth : A, float3 normal : B) : SV_TARGET
{
    float4 enc;
    enc.xy = EncodeViewNormalStereo (normal);
    enc.zw = EncodeFloatRG (depth);
    return enc;
}
