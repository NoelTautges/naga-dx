// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(half4 normal : COLOR0) : SV_TARGET
{
    // Quadratic polynomials
    half3 res = SHEvalLinearL2 (normal);

#   ifdef UNITY_COLORSPACE_GAMMA
        res = LinearToGammaSpace (res);
#   endif

    return res;
}
