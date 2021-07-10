/*
 * File: ASM_modal_controller_S3.c
 *
 * Code generated for Simulink model 'ASM_modal_controller_S3'.
 *
 * Model version                  : 1.1262
 * Simulink Coder version         : 9.0 (R2018b) 24-May-2018
 * C/C++ source code generated on : Tue Jul  6 09:20:13 2021
 *
 * Target selection: ert.tlc
 * Embedded hardware selection: Intel->x86-64 (Linux 64)
 * Code generation objectives: Unspecified
 * Validation result: Not run
 */

#include "ASM_modal_controller_S3.h"
#include "ASM_modal_controller_S3_private.h"

/* Block states (default storage) */
DW_ASM_modal_controller_S3_T ASM_modal_controller_S3_DW;

/* External inputs (root inport signals with default storage) */
ExtU_ASM_modal_controller_S3_T ASM_modal_controller_S3_U;

/* External outputs (root outports fed by signals with default storage) */
ExtY_ASM_modal_controller_S3_T ASM_modal_controller_S3_Y;

/* Real-time model */
RT_MODEL_ASM_modal_controller_S3_T ASM_modal_controller_S3_M_;
RT_MODEL_ASM_modal_controller_S3_T *const ASM_modal_controller_S3_M =
  &ASM_modal_controller_S3_M_;

/* Model step function */
void ASM_modal_controller_S3_step(void)
{
  real_T rtb_oldcmd[66];
  int32_T rtb_PulseGenerator;
  real_T rtb_deltacmd[66];
  real_T rtb_Polynomial;
  real_T rtb_Product2[66];
  real_T rtb_Product1[66];
  real_T rtb_Kb[66];
  int32_T i;
  int32_T i_0;
  real_T ASMPIcontroller_tmp[66];
  real_T rtb_deltacmd_j;
  real_T rtb_Product2_g;
  int32_T tmp;
  for (i = 0; i < 66; i++) {
    /* Sum: '<S3>/Sum' incorporates:
     *  Delay: '<S3>/Delay'
     *  Inport: '<Root>/ASM_cmd'
     */
    rtb_deltacmd[i] = ASM_modal_controller_S3_U.ASM_cmd[i] -
      ASM_modal_controller_S3_DW.Delay_DSTATE[i];

    /* Delay: '<S3>/Delay' */
    rtb_oldcmd[i] = ASM_modal_controller_S3_DW.Delay_DSTATE[i];
  }

  /* DiscretePulseGenerator: '<S3>/Pulse Generator' */
  rtb_PulseGenerator = ((ASM_modal_controller_S3_DW.clockTickCounter < 1) &&
                        (ASM_modal_controller_S3_DW.clockTickCounter >= 0));
  if (ASM_modal_controller_S3_DW.clockTickCounter >= 1) {
    ASM_modal_controller_S3_DW.clockTickCounter = 0;
  } else {
    ASM_modal_controller_S3_DW.clockTickCounter++;
  }

  /* End of DiscretePulseGenerator: '<S3>/Pulse Generator' */

  /* DiscreteIntegrator: '<S3>/Discrete-Time Integrator' */
  if ((rtb_PulseGenerator > 0) &&
      (ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_PrevResetState <= 0)) {
    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE = 0.0;
  }

  /* Polyval: '<S3>/Polynomial2' incorporates:
   *  DiscreteIntegrator: '<S3>/Discrete-Time Integrator'
   */
  rtb_Polynomial = ((6.1843584E+18 *
                     ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE +
                     -4.2166079999999995E+15) *
                    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE +
                    6.3888E+11) *
    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE;

  /* Product: '<S3>/Product2' */
  for (i = 0; i < 66; i++) {
    rtb_Product2[i] = rtb_deltacmd[i] * rtb_Polynomial;
  }

  /* End of Product: '<S3>/Product2' */

  /* Polyval: '<S3>/Polynomial1' incorporates:
   *  DiscreteIntegrator: '<S3>/Discrete-Time Integrator'
   */
  rtb_Polynomial = ((1.5460896E+18 *
                     ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE +
                     -1.4055359999999998E+15) *
                    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE +
                    3.1944E+11) *
    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE *
    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE;

  /* Product: '<S3>/Product1' */
  for (i = 0; i < 66; i++) {
    rtb_Product1[i] = rtb_deltacmd[i] * rtb_Polynomial;
  }

  /* End of Product: '<S3>/Product1' */

  /* Polyval: '<S3>/Polynomial' incorporates:
   *  Delay: '<S3>/Delay'
   *  DiscreteIntegrator: '<S3>/Discrete-Time Integrator'
   *  DiscreteTransferFcn: '<S1>/ASM PI controller'
   *  DiscreteTransferFcn: '<S1>/ASM PI controller'
   *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
   *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
   *  Polyval: '<S3>/Polynomial1'
   *  Polyval: '<S3>/Polynomial2'
   */
  rtb_Polynomial = 3.0921792E+17;
  for (i = 0; i < 5; i++) {
    rtb_Polynomial = rtb_Polynomial *
      ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE +
      ASM_modal_controller_S3_ConstP.Polynomial_Coefs[i + 1];
  }

  /* Outputs for IfAction SubSystem: '<S3>/If Action Subsystem1' incorporates:
   *  ActionPort: '<S7>/Action Port'
   */
  for (i = 0; i < 66; i++) {
    /* If: '<S3>/If' incorporates:
     *  Constant: '<S5>/Constant'
     *  DiscreteIntegrator: '<S3>/Discrete-Time Integrator'
     *  Gain: '<S7>/Gain'
     *  Inport: '<S6>/In3'
     *  Inport: '<S7>/delta'
     *  Product: '<S3>/Product'
     *  RelationalOperator: '<S5>/Compare'
     *  SignalConversion: '<S7>/OutportBufferForOut2'
     */
    rtb_Product2_g = rtb_Product2[i];
    rtb_deltacmd_j = rtb_deltacmd[i];
    if (ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE <=
        0.00045454545454545455) {
      /* Outputs for IfAction SubSystem: '<S3>/If Action Subsystem' incorporates:
       *  ActionPort: '<S6>/Action Port'
       */
      rtb_deltacmd_j = rtb_deltacmd[i] * rtb_Polynomial;

      /* End of Outputs for SubSystem: '<S3>/If Action Subsystem' */
    } else {
      rtb_Product2_g = 0.0 * rtb_deltacmd[i];
      rtb_Product1[i] = rtb_Product2_g;
    }

    rtb_Product2[i] = rtb_Product2_g;

    /* Outputs for IfAction SubSystem: '<S3>/If Action Subsystem' incorporates:
     *  ActionPort: '<S6>/Action Port'
     */
    rtb_deltacmd[i] = rtb_deltacmd_j;

    /* End of If: '<S3>/If' */
    /* End of Outputs for SubSystem: '<S3>/If Action Subsystem' */
  }

  /* End of Outputs for SubSystem: '<S3>/If Action Subsystem1' */
  for (i = 0; i < 66; i++) {
    /* Sum: '<S3>/Add' */
    rtb_deltacmd_j = rtb_deltacmd[i] + rtb_oldcmd[i];

    /* Gain: '<S1>/Kb' incorporates:
     *  Sum: '<S3>/Add'
     */
    rtb_Kb[i] = 33.6 * rtb_Product1[i];

    /* DiscreteTransferFcn: '<S1>/ASM PI controller' incorporates:
     *  Inport: '<Root>/ASM_FB'
     *  Sum: '<S1>/Sum1'
     *  Sum: '<S3>/Add'
     */
    rtb_Polynomial = (rtb_deltacmd_j - ASM_modal_controller_S3_U.ASM_FB[i]) -
      (-ASM_modal_controller_S3_DW.ASMPIcontroller_states[i]);
    ASMPIcontroller_tmp[i] = rtb_Polynomial;
    rtb_Polynomial *= 70031.657886510584;
    rtb_Product1[i] = -69968.342113489416 *
      ASM_modal_controller_S3_DW.ASMPIcontroller_states[i] + rtb_Polynomial;

    /* Sum: '<S3>/Add' */
    rtb_deltacmd[i] = rtb_deltacmd_j;

    /* Gain: '<S1>/Km' incorporates:
     *  Sum: '<S3>/Add'
     */
    rtb_oldcmd[i] = 0.0112 * rtb_Product2[i];
  }

  for (i = 0; i < 66; i++) {
    /* DiscreteTransferFcn: '<S1>/Numerical differentiation' incorporates:
     *  Delay: '<S3>/Delay'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  Inport: '<Root>/ASM_FB'
     *  Polyval: '<S3>/Polynomial'
     *  Polyval: '<S3>/Polynomial1'
     *  Polyval: '<S3>/Polynomial2'
     */
    rtb_Polynomial = ASM_modal_controller_S3_U.ASM_FB[i] - 0.22818630846850391 *
      ASM_modal_controller_S3_DW.Numericaldifferentiation_states[i];
    rtb_Product2[i] = rtb_Polynomial;
    rtb_Polynomial *= 9698.8968930214669;
    rtb_Polynomial += -9698.8968930214669 *
      ASM_modal_controller_S3_DW.Numericaldifferentiation_states[i];

    /* Outport: '<Root>/ASM_modalF' incorporates:
     *  Delay: '<S3>/Delay'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  Gain: '<S1>/Kd'
     *  Gain: '<S1>/Ks'
     *  Polyval: '<S3>/Polynomial'
     *  Polyval: '<S3>/Polynomial1'
     *  Polyval: '<S3>/Polynomial2'
     *  Sum: '<S1>/Sum2'
     *  Sum: '<S1>/Sum3'
     *  Sum: '<S1>/Sum4'
     *  Sum: '<S1>/Sum5'
     */
    rtb_Product2_g = 0.0;
    tmp = 0;
    for (i_0 = 0; i_0 < 66; i_0++) {
      rtb_Product2_g += ASM_modal_controller_S3_ConstP.Ks_Gain[tmp + i] *
        rtb_deltacmd[i_0];
      tmp += 66;
    }

    ASM_modal_controller_S3_Y.ASM_modalF[i] = ((rtb_oldcmd[i] + rtb_Kb[i]) +
      rtb_Product2_g) + (rtb_Product1[i] - 24.5 * rtb_Polynomial);

    /* End of Outport: '<Root>/ASM_modalF' */

    /* Outport: '<Root>/Mag_modal_vel' incorporates:
     *  Delay: '<S3>/Delay'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  Polyval: '<S3>/Polynomial'
     *  Polyval: '<S3>/Polynomial1'
     *  Polyval: '<S3>/Polynomial2'
     */
    ASM_modal_controller_S3_Y.Mag_modal_vel[i] = rtb_Polynomial;

    /* Update for Delay: '<S3>/Delay' incorporates:
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  Polyval: '<S3>/Polynomial'
     *  Polyval: '<S3>/Polynomial1'
     *  Polyval: '<S3>/Polynomial2'
     */
    ASM_modal_controller_S3_DW.Delay_DSTATE[i] =
      ASM_modal_controller_S3_DW.Delay_DSTATE[i + 66];
  }

  /* Update for DiscreteIntegrator: '<S3>/Discrete-Time Integrator' */
  ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_DSTATE += 0.000125;
  if (rtb_PulseGenerator > 0) {
    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_PrevResetState = 1;
  } else {
    ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_PrevResetState = 0;
  }

  /* End of Update for DiscreteIntegrator: '<S3>/Discrete-Time Integrator' */

  /* Update for Delay: '<S3>/Delay' incorporates:
   *  Inport: '<Root>/ASM_cmd'
   */
  memcpy(&ASM_modal_controller_S3_DW.Delay_DSTATE[66],
         &ASM_modal_controller_S3_U.ASM_cmd[0], 66U * sizeof(real_T));

  /* Update for DiscreteTransferFcn: '<S1>/ASM PI controller' incorporates:
   *  Delay: '<S3>/Delay'
   */
  memcpy(&ASM_modal_controller_S3_DW.ASMPIcontroller_states[0],
         &ASMPIcontroller_tmp[0], 66U * sizeof(real_T));

  /* Update for DiscreteTransferFcn: '<S1>/Numerical differentiation' incorporates:
   *  Delay: '<S3>/Delay'
   */
  memcpy(&ASM_modal_controller_S3_DW.Numericaldifferentiation_states[0],
         &rtb_Product2[0], 66U * sizeof(real_T));
}

/* Model initialize function */
void ASM_modal_controller_S3_initialize(void)
{
  /* Registration code */

  /* initialize error status */
  rtmSetErrorStatus(ASM_modal_controller_S3_M, (NULL));

  /* states (dwork) */
  (void) memset((void *)&ASM_modal_controller_S3_DW, 0,
                sizeof(DW_ASM_modal_controller_S3_T));

  /* external inputs */
  (void)memset(&ASM_modal_controller_S3_U, 0, sizeof
               (ExtU_ASM_modal_controller_S3_T));

  /* external outputs */
  (void) memset((void *)&ASM_modal_controller_S3_Y, 0,
                sizeof(ExtY_ASM_modal_controller_S3_T));

  /* InitializeConditions for DiscretePulseGenerator: '<S3>/Pulse Generator' */
  ASM_modal_controller_S3_DW.clockTickCounter = 0;

  /* InitializeConditions for DiscreteIntegrator: '<S3>/Discrete-Time Integrator' */
  ASM_modal_controller_S3_DW.DiscreteTimeIntegrator_PrevResetState = 2;
}

/* Model terminate function */
void ASM_modal_controller_S3_terminate(void)
{
  /* (no terminate code required) */
}

/*
 * File trailer for generated code.
 *
 * [EOF]
 */
