// SM: 4_1, 5_0

#include "UnityCG.cginc"

v2f_img PSMain(appdata_img v : COLOR0) : SV_TARGET
{
    v2f_img o;
    UNITY_INITIALIZE_OUTPUT(v2f_img, o);
    UNITY_SETUP_INSTANCE_ID(v);
    UNITY_INITIALIZE_VERTEX_OUTPUT_STEREO(o);

    o.pos = UnityObjectToClipPos (v.vertex);
    o.uv = v.texcoord;
    return o;
}
