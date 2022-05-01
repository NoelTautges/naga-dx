// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(half4 normal : COLOR0) : SV_TARGET
{
    half3 x;

    // Linear (L1) + constant (L0) polynomial terms
    x.r = dot(unity_SHAr,normal);
    x.g = dot(unity_SHAg,normal);
    x.b = dot(unity_SHAb,normal);

    return x;
}
