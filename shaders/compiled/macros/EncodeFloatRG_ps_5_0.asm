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
// SV_TARGET                0   xy          0   TARGET   float   xy  
//
ps_5_0
dcl_globalFlags refactoringAllowed
dcl_input_ps linear v0.x
dcl_output o0.xy
dcl_temps 1
mul r0.xy, v0.xxxx, l(1.000000, 255.000000, 0.000000, 0.000000)
frc r0.yz, r0.xxyx
mad r0.x, -r0.z, l(0.003922), r0.y
mov o0.xy, r0.xzxx
ret 
// Approximately 5 instruction slots used
