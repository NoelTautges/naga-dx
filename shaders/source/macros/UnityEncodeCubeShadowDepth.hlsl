// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float z : A) : SV_TARGET
{
    #ifdef UNITY_USE_RGBA_FOR_POINT_SHADOWS
    return EncodeFloatRGBA (min(z, 0.999));
    #else
    return z;
    #endif
}
