//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// A                        0   xyz         0     NONE   float   xyz 
// B                        0      w        0     NONE   float      w
//
//
// Output signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_TARGET                0   xyzw        0   TARGET   float   xyzw
//
ps_4_1
dcl_globalFlags refactoringAllowed
dcl_input_ps linear v0.xyz
dcl_input_ps linear v0.w
dcl_output o0.xyzw
dcl_temps 2
div r0.x, l(1.000000, 1.000000, 1.000000, 1.000000), v0.w
mul r0.xyz, r0.xxxx, v0.xyzx
max r0.w, r0.y, r0.x
max r1.x, r0.z, l(0.020000)
max r0.w, r0.w, r1.x
mul r0.w, r0.w, l(255.000000)
round_pi r0.w, r0.w
mul r0.w, r0.w, l(0.003922)
div o0.xyz, r0.xyzx, r0.wwww
mov o0.w, r0.w
ret 
// Approximately 11 instruction slots used
