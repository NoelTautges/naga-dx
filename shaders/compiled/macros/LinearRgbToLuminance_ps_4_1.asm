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
dcl_input_ps linear v0.xyz
dcl_output o0.x
dp3 o0.x, v0.xyzx, l(0.212673, 0.715152, 0.072175, 0.000000)
ret 
// Approximately 2 instruction slots used
