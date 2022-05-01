//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
// Buffer Definitions: 
//
// cbuffer $Globals
// {
//
//   float4 _Time;                      // Offset:    0 Size:    16 [unused]
//   float4 _SinTime;                   // Offset:   16 Size:    16 [unused]
//   float4 _CosTime;                   // Offset:   32 Size:    16 [unused]
//   float4 unity_DeltaTime;            // Offset:   48 Size:    16 [unused]
//   float3 _WorldSpaceCameraPos;       // Offset:   64 Size:    12 [unused]
//   float4 _ProjectionParams;          // Offset:   80 Size:    16 [unused]
//   float4 _ScreenParams;              // Offset:   96 Size:    16 [unused]
//   float4 _ZBufferParams;             // Offset:  112 Size:    16 [unused]
//   float4 unity_OrthoParams;          // Offset:  128 Size:    16 [unused]
//   float4 unity_CameraWorldClipPlanes[6];// Offset:  144 Size:    96 [unused]
//   float4x4 unity_CameraProjection;   // Offset:  240 Size:    64 [unused]
//   float4x4 unity_CameraInvProjection;// Offset:  304 Size:    64 [unused]
//   float4x4 unity_WorldToCamera;      // Offset:  368 Size:    64 [unused]
//   float4x4 unity_CameraToWorld;      // Offset:  432 Size:    64 [unused]
//   float4 _WorldSpaceLightPos0;       // Offset:  496 Size:    16 [unused]
//   float4 _LightPositionRange;        // Offset:  512 Size:    16 [unused]
//   float4 _LightProjectionParams;     // Offset:  528 Size:    16 [unused]
//   float4 unity_4LightPosX0;          // Offset:  544 Size:    16 [unused]
//   float4 unity_4LightPosY0;          // Offset:  560 Size:    16 [unused]
//   float4 unity_4LightPosZ0;          // Offset:  576 Size:    16 [unused]
//   float4 unity_4LightAtten0;         // Offset:  592 Size:    16 [unused]
//   float4 unity_LightColor[8];        // Offset:  608 Size:   128
//   float4 unity_LightPosition[8];     // Offset:  736 Size:   128
//   float4 unity_LightAtten[8];        // Offset:  864 Size:   128
//   float4 unity_SpotDirection[8];     // Offset:  992 Size:   128 [unused]
//   float4 unity_SHAr;                 // Offset: 1120 Size:    16 [unused]
//   float4 unity_SHAg;                 // Offset: 1136 Size:    16 [unused]
//   float4 unity_SHAb;                 // Offset: 1152 Size:    16 [unused]
//   float4 unity_SHBr;                 // Offset: 1168 Size:    16 [unused]
//   float4 unity_SHBg;                 // Offset: 1184 Size:    16 [unused]
//   float4 unity_SHBb;                 // Offset: 1200 Size:    16 [unused]
//   float4 unity_SHC;                  // Offset: 1216 Size:    16 [unused]
//   float4 unity_OcclusionMaskSelector;// Offset: 1232 Size:    16 [unused]
//   float4 unity_ProbesOcclusion;      // Offset: 1248 Size:    16 [unused]
//   float3 unity_LightColor0;          // Offset: 1264 Size:    12 [unused]
//   float3 unity_LightColor1;          // Offset: 1280 Size:    12 [unused]
//   float3 unity_LightColor2;          // Offset: 1296 Size:    12 [unused]
//   float3 unity_LightColor3;          // Offset: 1312 Size:    12 [unused]
//   float4 unity_ShadowSplitSpheres[4];// Offset: 1328 Size:    64 [unused]
//   float4 unity_ShadowSplitSqRadii;   // Offset: 1392 Size:    16 [unused]
//   float4 unity_LightShadowBias;      // Offset: 1408 Size:    16 [unused]
//   float4 _LightSplitsNear;           // Offset: 1424 Size:    16 [unused]
//   float4 _LightSplitsFar;            // Offset: 1440 Size:    16 [unused]
//   float4x4 unity_WorldToShadow[4];   // Offset: 1456 Size:   256 [unused]
//   float4 _LightShadowData;           // Offset: 1712 Size:    16 [unused]
//   float4 unity_ShadowFadeCenterAndType;// Offset: 1728 Size:    16 [unused]
//   float4x4 unity_ObjectToWorld;      // Offset: 1744 Size:    64
//   float4x4 unity_WorldToObject;      // Offset: 1808 Size:    64
//   float4 unity_LODFade;              // Offset: 1872 Size:    16 [unused]
//   float4 unity_WorldTransformParams; // Offset: 1888 Size:    16 [unused]
//   float4x4 glstate_matrix_transpose_modelview0;// Offset: 1904 Size:    64 [unused]
//   float4 glstate_lightmodel_ambient; // Offset: 1968 Size:    16
//   float4 unity_AmbientSky;           // Offset: 1984 Size:    16 [unused]
//   float4 unity_AmbientEquator;       // Offset: 2000 Size:    16 [unused]
//   float4 unity_AmbientGround;        // Offset: 2016 Size:    16 [unused]
//   float4 unity_IndirectSpecColor;    // Offset: 2032 Size:    16 [unused]
//   float4x4 glstate_matrix_projection;// Offset: 2048 Size:    64 [unused]
//   float4x4 unity_MatrixV;            // Offset: 2112 Size:    64
//   float4x4 unity_MatrixInvV;         // Offset: 2176 Size:    64
//   float4x4 unity_MatrixVP;           // Offset: 2240 Size:    64 [unused]
//   int unity_StereoEyeIndex;          // Offset: 2304 Size:     4 [unused]
//   float4 unity_ShadowColor;          // Offset: 2320 Size:    16 [unused]
//   float4 unity_FogColor;             // Offset: 2336 Size:    16 [unused]
//   float4 unity_FogParams;            // Offset: 2352 Size:    16 [unused]
//   float4 unity_LightmapST;           // Offset: 2368 Size:    16 [unused]
//   float4 unity_DynamicLightmapST;    // Offset: 2384 Size:    16 [unused]
//   float4 unity_SpecCube0_BoxMax;     // Offset: 2400 Size:    16 [unused]
//   float4 unity_SpecCube0_BoxMin;     // Offset: 2416 Size:    16 [unused]
//   float4 unity_SpecCube0_ProbePosition;// Offset: 2432 Size:    16 [unused]
//   float4 unity_SpecCube0_HDR;        // Offset: 2448 Size:    16 [unused]
//   float4 unity_SpecCube1_BoxMax;     // Offset: 2464 Size:    16 [unused]
//   float4 unity_SpecCube1_BoxMin;     // Offset: 2480 Size:    16 [unused]
//   float4 unity_SpecCube1_ProbePosition;// Offset: 2496 Size:    16 [unused]
//   float4 unity_SpecCube1_HDR;        // Offset: 2512 Size:    16 [unused]
//   float4 unity_Lightmap_HDR;         // Offset: 2528 Size:    16 [unused]
//   float4 unity_DynamicLightmap_HDR;  // Offset: 2544 Size:    16 [unused]
//
// }
//
//
// Resource Bindings:
//
// Name                                 Type  Format         Dim      HLSL Bind  Count
// ------------------------------ ---------- ------- ----------- -------------- ------
// $Globals                          cbuffer      NA          NA            cb0      1 
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// COLOR                    0   xyzw        0     NONE   float   xyz 
// COLOR                    1   xyz         1     NONE   float   xyz 
//
//
// Output signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_TARGET                0   xyz         0   TARGET   float   xyz 
//
ps_4_1
dcl_globalFlags refactoringAllowed
dcl_constantbuffer CB0[139], dynamicIndexed
dcl_input_ps linear v0.xyz
dcl_input_ps linear v1.xyz
dcl_output o0.xyz
dcl_temps 5
mul r0.xyz, cb0[114].xyzx, cb0[136].yyyy
mad r0.xyz, cb0[113].xyzx, cb0[136].xxxx, r0.xyzx
mad r0.xyz, cb0[115].xyzx, cb0[136].zzzz, r0.xyzx
mad r0.xyz, cb0[116].xyzx, cb0[136].wwww, r0.xyzx
mul r1.xyz, cb0[114].xyzx, cb0[137].yyyy
mad r1.xyz, cb0[113].xyzx, cb0[137].xxxx, r1.xyzx
mad r1.xyz, cb0[115].xyzx, cb0[137].zzzz, r1.xyzx
mad r1.xyz, cb0[116].xyzx, cb0[137].wwww, r1.xyzx
mul r2.xyz, cb0[114].xyzx, cb0[138].yyyy
mad r2.xyz, cb0[113].xyzx, cb0[138].xxxx, r2.xyzx
mad r2.xyz, cb0[115].xyzx, cb0[138].zzzz, r2.xyzx
mad r2.xyz, cb0[116].xyzx, cb0[138].wwww, r2.xyzx
mul r3.xyzw, v0.yyyy, cb0[110].xyzw
mad r3.xyzw, cb0[109].xyzw, v0.xxxx, r3.xyzw
mad r3.xyzw, cb0[111].xyzw, v0.zzzz, r3.xyzw
add r3.xyzw, r3.xyzw, cb0[112].xyzw
mul r4.xyz, r3.yyyy, cb0[133].xyzx
mad r4.xyz, cb0[132].xyzx, r3.xxxx, r4.xyzx
mad r3.xyz, cb0[134].xyzx, r3.zzzz, r4.xyzx
mad r3.xyz, cb0[135].xyzx, r3.wwww, r3.xyzx
dp3 r0.x, r0.xyzx, v1.xyzx
dp3 r0.y, r1.xyzx, v1.xyzx
dp3 r0.z, r2.xyzx, v1.xyzx
dp3 r0.w, r0.xyzx, r0.xyzx
rsq r0.w, r0.w
mul r0.xyz, r0.wwww, r0.xyzx
add r1.xyz, cb0[123].xyzx, cb0[123].xyzx
mov r2.xyz, r1.xyzx
mov r0.w, l(0)
loop 
  ige r1.w, r0.w, l(4)
  breakc_nz r1.w
  mad r4.xyz, -r3.xyzx, cb0[r0.w + 46].wwww, cb0[r0.w + 46].xyzx
  dp3 r1.w, r4.xyzx, r4.xyzx
  max r1.w, r1.w, l(0.000001)
  rsq r2.w, r1.w
  mul r4.xyz, r2.wwww, r4.xyzx
  mad r1.w, r1.w, cb0[r0.w + 54].z, l(1.000000)
  div r1.w, l(1.000000, 1.000000, 1.000000, 1.000000), r1.w
  dp3 r2.w, r0.xyzx, r4.xyzx
  max r2.w, r2.w, l(0.000000)
  mul r1.w, r1.w, r2.w
  mad r2.xyz, cb0[r0.w + 38].xyzx, r1.wwww, r2.xyzx
  iadd r0.w, r0.w, l(1)
endloop 
mov o0.xyz, r2.xyzx
ret 
// Approximately 47 instruction slots used
