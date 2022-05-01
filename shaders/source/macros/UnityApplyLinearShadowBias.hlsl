// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float4 clipPos : COLOR0) : SV_TARGET
{
#if defined(UNITY_REVERSED_Z)

    // For point lights that support depth cube map, the bias is applied in the fragment shader sampling the shadow map.
    // This is because the legacy behaviour for point light shadow map cannot be implemented by offseting the vertex position
    // in the vertex shader generating the shadow map.
#   if !(defined(SHADOWS_CUBE) && defined(SHADOWS_CUBE_IN_DEPTH_TEX))
    // We use max/min instead of clamp to ensure proper handling of the rare case
    // where both numerator and denominator are zero and the fraction becomes NaN.
    clipPos.z += max(-1, min(unity_LightShadowBias.x / clipPos.w, 0));
#   endif
    float clamped = min(clipPos.z, clipPos.w*UNITY_NEAR_CLIP_VALUE);
#else
    clipPos.z += saturate(unity_LightShadowBias.x/clipPos.w);
    float clamped = max(clipPos.z, clipPos.w*UNITY_NEAR_CLIP_VALUE);
#endif
    clipPos.z = lerp(clipPos.z, clamped, unity_LightShadowBias.y);
    return clipPos;
}
