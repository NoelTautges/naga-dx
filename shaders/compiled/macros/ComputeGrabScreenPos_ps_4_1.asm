//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
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
// SV_TARGET                0   xyzw        0   TARGET   float   xyzw
//
ps_4_1
dcl_globalFlags refactoringAllowed
dcl_input_ps linear v0.xyzw
dcl_output o0.xyzw
dcl_temps 1
mul r0.xyz, v0.xywx, l(0.500000, 0.500000, 0.500000, 0.000000)
add o0.xy, r0.zzzz, r0.xyxx
mov o0.zw, v0.zzzw
ret 
// Approximately 4 instruction slots used
