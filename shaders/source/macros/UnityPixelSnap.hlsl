// SM: 4_1, 5_0

#include "UnityCG.cginc"

float4 PSMain(float4 pos : COLOR0) : SV_TARGET
{
    float2 hpc = _ScreenParams.xy * 0.5f;
#if  SHADER_API_PSSL
// sdk 4.5 splits round into v_floor_f32(x+0.5) ... sdk 5.0 uses v_rndne_f32, for compatabilty we use the 4.5 version
    float2 temp = ((pos.xy / pos.w) * hpc) + float2(0.5f,0.5f);
    float2 pixelPos = float2(__v_floor_f32(temp.x), __v_floor_f32(temp.y));
#else
    float2 pixelPos = round ((pos.xy / pos.w) * hpc);
#endif
    pos.xy = pixelPos / hpc * pos.w;
    return pos;
}
