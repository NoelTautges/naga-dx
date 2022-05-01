// SM: 4_1, 5_0

#include "UnityCG.cginc"

fixed4 PSMain(v2f_vertex_lit i : COLOR0, sampler2D mainTex : COLOR1) : SV_TARGET
{
    fixed4 texcol = tex2D( mainTex, i.uv );
    fixed4 c;
    c.xyz = ( texcol.xyz * i.diff.xyz + i.spec.xyz * texcol.a );
    c.w = texcol.w * i.diff.w;
    return c;
}
