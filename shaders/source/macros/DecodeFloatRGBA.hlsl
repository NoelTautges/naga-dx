// SM: 4_1, 5_0

#include "UnityCG.cginc"

float PSMain(float4 enc : A) : SV_TARGET
{
    float4 kDecodeDot = float4(1.0, 1/255.0, 1/65025.0, 1/16581375.0);
    return dot( enc, kDecodeDot );
}
