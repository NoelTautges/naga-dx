//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// A                        0   xyzw        0     NONE   float   xy  
// A                        1   xyzw        1     NONE   float   xy  
// A                        2   xyzw        2     NONE   float       
// A                        3   xyzw        3     NONE   float       
// B                        0   xy          4     NONE   float   xy  
//
//
// Output signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// SV_TARGET                0   xy          0   TARGET   float   xy  
//
ps_4_1
dcl_globalFlags refactoringAllowed
dcl_input_ps linear v0.xy
dcl_input_ps linear v1.xy
dcl_input_ps linear v4.xy
dcl_output o0.xy
dcl_temps 1
mul r0.xy, v1.xyxx, v4.yyyy
mad o0.xy, v0.xyxx, v4.xxxx, r0.xyxx
ret 
// Approximately 3 instruction slots used
