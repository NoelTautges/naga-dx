// SM: 4_1, 5_0

#include "UnityCG.cginc"

float PSMain(float z : COLOR0) : SV_TARGET
{
    return 1.0 / (_ZBufferParams.z * z + _ZBufferParams.w);
}
