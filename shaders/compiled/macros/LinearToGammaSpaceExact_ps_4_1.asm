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
// SV_TARGET                0   x           0   TARGET   float   x   
//
ps_4_1
dcl_globalFlags refactoringAllowed
dcl_input_ps linear v0.x
dcl_output o0.x
dcl_temps 1
ge r0.x, l(0.000000), v0.x
if_nz r0.x
  mov o0.x, l(0)
  ret 
else 
  ge r0.x, l(0.003131), v0.x
  if_nz r0.x
    mul o0.x, v0.x, l(12.920000)
    ret 
  else 
    lt r0.x, v0.x, l(1.000000)
    if_nz r0.x
      log r0.x, v0.x
      mul r0.x, r0.x, l(0.416667)
      exp r0.x, r0.x
      mad o0.x, r0.x, l(1.055000), l(-0.055000)
      ret 
    else 
      log r0.x, v0.x
      mul r0.x, r0.x, l(0.454545)
      exp o0.x, r0.x
      ret 
    endif 
  endif 
endif 
ret 
// Approximately 26 instruction slots used
