// SM: 4_1, 5_0

#include "UnityCG.cginc"

fixed3 PSMain(fixed4 packednormal : COLOR0) : SV_TARGET
{
    // This do the trick
   packednormal.x *= packednormal.w;

    fixed3 normal;
    normal.xy = packednormal.xy * 2 - 1;
    normal.z = sqrt(1 - saturate(dot(normal.xy, normal.xy)));
    return normal;
}
