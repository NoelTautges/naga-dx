// SM: 4_1, 5_0

#include "UnityCG.cginc"

half4 PSMain(half3 color : A, float maxRGBM : B) : SV_TARGET
{
    float kOneOverRGBMMaxRange = 1.0 / maxRGBM;
    const float kMinMultiplier = 2.0 * 1e-2;

    float3 rgb = color * kOneOverRGBMMaxRange;
    float alpha = max(max(rgb.r, rgb.g), max(rgb.b, kMinMultiplier));
    alpha = ceil(alpha * 255.0) / 255.0;

    // Division-by-zero warning from d3d9, so make compiler happy.
    alpha = max(alpha, kMinMultiplier);

    return half4(rgb / alpha, alpha);
}
