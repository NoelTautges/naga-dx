//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_Position              0   xyz         0     NONE   float   xyz 
// PSIZE                    0   x           1     NONE   float   x   
//
//
// Output signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_Position              0   xyzw        0      POS   float   xyzw
//
vs_5_0
dcl_globalFlags refactoringAllowed
dcl_input v0.xyz
dcl_input v1.x
dcl_output_siv o0.xyzw, position
mov o0.xyz, v0.xyzx
mov o0.w, v1.x
ret 
// Approximately 3 instruction slots used
