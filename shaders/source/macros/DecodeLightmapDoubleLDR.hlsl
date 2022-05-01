// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(fixed4 color : COLOR0, half4 decodeInstructions : COLOR1) : SV_TARGET
{
    // decodeInstructions.x contains 2.0 when gamma color space is used or pow(2.0, 2.2) = 4.59 when linear color space is used on mobile platforms
    return decodeInstructions.x * color.rgb;
}
