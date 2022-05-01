// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(in float3 worldPos : COLOR0) : SV_TARGET
{
    return _WorldSpaceCameraPos.xyz - worldPos;
}
