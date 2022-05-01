// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(float4 enc4 : COLOR0) : SV_TARGET
{
    float kScale = 1.7777;
    float3 nn = enc4.xyz*float3(2*kScale,2*kScale,0) + float3(-kScale,-kScale,1);
    float g = 2.0 / dot(nn.xyz,nn.xyz);
    float3 n;
    n.xy = g*nn.xy;
    n.z = g-1;
    return n;
}
