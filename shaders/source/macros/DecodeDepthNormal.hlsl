// SM: 4_1, 5_0

#include "UnityCG.cginc"

void PSMain(float4 enc : A, out float depth : COLOR1, out float3 normal : COLOR2)
{
    depth = DecodeFloatRG (enc.zw);
    normal = DecodeViewNormalStereo (enc);
}
