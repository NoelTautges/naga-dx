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
//
//
// Output signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_TARGET                0   x           0   TARGET   float   x   
//
ps_4_1
dcl_globalFlags refactoringAllowed
dcl_input_ps linear v0.xyzw
dcl_output o0.x
dp4 o0.x, v0.xyzw, l(1.000000, 0.003922, 0.000015, 0.000000)
ret 
// Approximately 2 instruction slots used
