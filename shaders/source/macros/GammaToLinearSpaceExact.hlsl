// SM: 4_1, 5_0

#include "UnityCG.cginc"

float PSMain(float value : A) : SV_TARGET
{
    if (value <= 0.04045F)
        return value / 12.92F;
    else if (value < 1.0F)
        return pow((value + 0.055F)/1.055F, 2.4F);
    else
        return pow(value, 2.2F);
}
