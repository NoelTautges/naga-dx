// SM: 4_1, 5_0

#include "UnityCG.cginc"

half3 PSMain(fixed4 color : COLOR0) : SV_TARGET
{
    //@TODO: Temporary until Geomerics gives us an API to convert lightmaps to RGBM in gamma space on the enlighten thread before we upload the textures.
#if defined(UNITY_FORCE_LINEAR_READ_FOR_RGBM)
    return pow ((unity_DynamicLightmap_HDR.x * color.a) * sqrt(color.rgb), unity_DynamicLightmap_HDR.y);
#else
    return pow ((unity_DynamicLightmap_HDR.x * color.a) * color.rgb, unity_DynamicLightmap_HDR.y);
#endif
}
