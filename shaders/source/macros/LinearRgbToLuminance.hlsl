// SM: 4_1, 5_0

#include "UnityCG.cginc"

half PSMain(half3 linearRgb : COLOR0) : SV_TARGET
{
    return dot(linearRgb, half3(0.2126729f,  0.7151522f, 0.0721750f));
}
