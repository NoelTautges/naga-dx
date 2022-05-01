// SM: 4_1, 5_0

#include "UnityCG.cginc"

void PSMain(float4 enc : COLOR0, out float depth : COLOR1, out float3 normal : COLOR2) : SV_TARGET
{
    depth = DecodeFloatRG (enc.zw);
    normal = DecodeViewNormalStereo (enc);
}
