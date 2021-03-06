//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// A                        0   xyzw        0     NONE   float   xyzw
// B                        0   xyzw        1     NONE   float   xy  
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
dcl_input_ps linear v0.xyzw
dcl_input_ps linear v1.xy
dcl_output o0.xyz
dcl_temps 1
log r0.x, v0.w
mul r0.x, r0.x, v1.y
exp r0.x, r0.x
mul r0.x, r0.x, v1.x
mul o0.xyz, r0.xxxx, v0.xyzx
ret 
// Approximately 6 instruction slots used
