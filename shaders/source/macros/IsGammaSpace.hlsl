// SM: 4_1, 5_0

#include "UnityCG.cginc"

bool PSMain() : SV_TARGET
{
    #ifdef UNITY_COLORSPACE_GAMMA
        return true;
    #else
        return false;
    #endif
}
