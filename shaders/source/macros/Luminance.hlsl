// SM: 4_1, 5_0

#include "UnityCG.cginc"

half PSMain(half3 rgb : A) : SV_TARGET
{
    return dot(rgb, unity_ColorSpaceLuminance.rgb);
}
