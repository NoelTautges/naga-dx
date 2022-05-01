//
// Generated by Microsoft (R) HLSL Shader Compiler 10.1
//
//
//
// Input signature:
//
// Name                 Index   Mask Register SysValue  Format   Used
// -------------------- ----- ------ -------- -------- ------- ------
// COLOR                    0   x           0     NONE   float   x   
// COLOR                    1    y          0     NONE   float    y  
// COLOR                    2   xyz         1     NONE   float   xyz 
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
dcl_input_ps linear v0.y
dcl_input_ps linear v1.xyz
dcl_output o0.xy
dcl_temps 1
dp3 r0.x, v1.xyzx, v1.xyzx
rsq r0.x, r0.x
mul r0.yz, r0.xxxx, v1.xxyx
mad r0.x, v1.z, r0.x, l(0.420000)
div r0.xy, r0.yzyy, r0.xxxx
mul r0.z, v0.y, l(0.500000)
mad r0.z, v0.x, v0.y, -r0.z
mul o0.xy, r0.xyxx, r0.zzzz
ret 
// Approximately 9 instruction slots used
