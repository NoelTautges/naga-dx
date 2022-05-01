// SM: 4_1, 5_0

#include "UnityCG.cginc"

fixed3 PSMain(fixed4 packednormal : A) : SV_TARGET
{
#if defined(UNITY_NO_DXT5nm)
    return packednormal.xyz * 2 - 1;
#else
    return UnpackNormalmapRGorAG(packednormal);
#endif
}
