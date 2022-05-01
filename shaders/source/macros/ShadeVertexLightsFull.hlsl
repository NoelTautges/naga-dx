// SM: 4_1, 5_0

#include "UnityCG.cginc"

float3 PSMain(float4 vertex : COLOR0, float3 normal : COLOR1, int lightCount : COLOR2, bool spotLight : COLOR3) : SV_TARGET
{
    float3 viewpos = UnityObjectToViewPos (vertex);
    float3 viewN = normalize (mul ((float3x3)UNITY_MATRIX_IT_MV, normal));

    float3 lightColor = UNITY_LIGHTMODEL_AMBIENT.xyz;
    for (int i = 0; i < lightCount; i++) {
        float3 toLight = unity_LightPosition[i].xyz - viewpos.xyz * unity_LightPosition[i].w;
        float lengthSq = dot(toLight, toLight);

        // don't produce NaNs if some vertex position overlaps with the light
        lengthSq = max(lengthSq, 0.000001);

        toLight *= rsqrt(lengthSq);

        float atten = 1.0 / (1.0 + lengthSq * unity_LightAtten[i].z);
        if (spotLight)
        {
            float rho = max (0, dot(toLight, unity_SpotDirection[i].xyz));
            float spotAtt = (rho - unity_LightAtten[i].x) * unity_LightAtten[i].y;
            atten *= saturate(spotAtt);
        }

        float diff = max (0, dot (viewN, toLight));
        lightColor += unity_LightColor[i].rgb * (diff * atten);
    }
    return lightColor;
}
