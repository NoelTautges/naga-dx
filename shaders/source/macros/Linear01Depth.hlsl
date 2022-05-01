// SM: 4_1, 5_0

#include "UnityCG.cginc"

float PSMain(float z : A) : SV_TARGET
{
    return 1.0 / (_ZBufferParams.x * z + _ZBufferParams.y);
}
