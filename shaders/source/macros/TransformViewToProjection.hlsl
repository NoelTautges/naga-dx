// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(float3 v : A) : SV_TARGET
{
    return mul((float3x3)UNITY_MATRIX_P, v);
}
