// SM: 4_1, 5_0

#include "UnityCG.cginc"

float2 PSMain(float v : COLOR0) : SV_TARGET
{
    float2 kEncodeMul = float2(1.0, 255.0);
    float kEncodeBit = 1.0/255.0;
    float2 enc = kEncodeMul * v;
    enc = frac (enc);
    enc.x -= enc.y * kEncodeBit;
    return enc;
}
