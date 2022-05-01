// SM: 4_1, 5_0

#include "UnityCG.cginc"

float PSMain(float4 vals : A) : SV_TARGET
{
    #ifdef UNITY_USE_RGBA_FOR_POINT_SHADOWS
    return DecodeFloatRGBA (vals);
    #else
    return vals.r;
    #endif
}
