// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(half4 data : A, half4 decodeInstructions : B) : SV_TARGET
{
    // If Linear mode is not supported we can skip exponent part
    #if defined(UNITY_COLORSPACE_GAMMA)
    # if defined(UNITY_FORCE_LINEAR_READ_FOR_RGBM)
        return (decodeInstructions.x * data.a) * sqrt(data.rgb);
    # else
        return (decodeInstructions.x * data.a) * data.rgb;
    # endif
    #else
        return (decodeInstructions.x * pow(data.a, decodeInstructions.y)) * data.rgb;
    #endif
}
