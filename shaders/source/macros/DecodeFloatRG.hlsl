// SM: 4_1, 5_0

#include "UnityCG.cginc"

float PSMain(float2 enc : A) : SV_TARGET
{
    float2 kDecodeDot = float2(1.0, 1/255.0);
    return dot( enc, kDecodeDot );
}
