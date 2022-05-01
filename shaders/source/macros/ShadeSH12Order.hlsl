// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(half4 normal : A) : SV_TARGET
{
    // Linear + constant polynomial terms
    half3 res = SHEvalLinearL0L1 (normal);

#   ifdef UNITY_COLORSPACE_GAMMA
        res = LinearToGammaSpace (res);
#   endif

    return res;
}
