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
//   float4 unity_LightColor[8];        // Offset:  608 Size:   128 [unused]
//   float4 unity_LightPosition[8];     // Offset:  736 Size:   128 [unused]
//   float4 unity_LightAtten[8];        // Offset:  864 Size:   128 [unused]
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
//   float4x4 unity_ObjectToWorld;      // Offset: 1744 Size:    64 [unused]
//   float4x4 unity_WorldToObject;      // Offset: 1808 Size:    64 [unused]
//   float4 unity_LODFade;              // Offset: 1872 Size:    16 [unused]
//   float4 unity_WorldTransformParams; // Offset: 1888 Size:    16 [unused]
//   float4x4 glstate_matrix_transpose_modelview0;// Offset: 1904 Size:    64 [unused]
//   float4 glstate_lightmodel_ambient; // Offset: 1968 Size:    16 [unused]
//   float4 unity_AmbientSky;           // Offset: 1984 Size:    16 [unused]
//   float4 unity_AmbientEquator;       // Offset: 2000 Size:    16 [unused]
//   float4 unity_AmbientGround;        // Offset: 2016 Size:    16 [unused]
//   float4 unity_IndirectSpecColor;    // Offset: 2032 Size:    16 [unused]
//   float4x4 glstate_matrix_projection;// Offset: 2048 Size:    64 [unused]
//   float4x4 unity_MatrixV;            // Offset: 2112 Size:    64 [unused]
//   float4x4 unity_MatrixInvV;         // Offset: 2176 Size:    64 [unused]
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
//   float4 unity_DynamicLightmap_HDR;  // Offset: 2544 Size:    16
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
// COLOR                    0   xyzw        0     NONE   float   xyzw
//
//
// Output signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_TARGET                0   xyz         0   TARGET   float   xyz 
//
ps_5_0
dcl_globalFlags refactoringAllowed
dcl_constantbuffer CB0[160], immediateIndexed
dcl_input_ps linear v0.xyzw
dcl_output o0.xyz
dcl_temps 1
mul r0.x, v0.w, cb0[159].x
mul r0.xyz, r0.xxxx, v0.xyzx
log r0.xyz, r0.xyzx
mul r0.xyz, r0.xyzx, cb0[159].yyyy
exp o0.xyz, r0.xyzx
ret 
// Approximately 6 instruction slots used
