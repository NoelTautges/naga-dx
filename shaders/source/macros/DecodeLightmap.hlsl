// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(fixed4 color : A) : SV_TARGET
{
    return DecodeLightmap( color, unity_Lightmap_HDR );
}
