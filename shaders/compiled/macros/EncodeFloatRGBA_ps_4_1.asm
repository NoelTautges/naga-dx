//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// A                        0   x           0     NONE   float   x   
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
dcl_input_ps linear v0.x
dcl_output o0.xyzw
dcl_temps 1
mul r0.xyzw, v0.xxxx, l(1.000000, 255.000000, 65025.000000, 16581375.000000)
frc r0.xyzw, r0.xyzw
mad o0.xyzw, -r0.yzww, l(0.003922, 0.003922, 0.003922, 0.003922), r0.xyzw
ret 
// Approximately 4 instruction slots used
