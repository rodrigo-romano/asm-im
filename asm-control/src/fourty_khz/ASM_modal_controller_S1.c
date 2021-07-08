/*
 * File: ASM_modal_controller_S1.c
 *
 * Code generated for Simulink model 'ASM_modal_controller_S1'.
 *
 * Model version                  : 1.1269
 * Simulink Coder version         : 9.0 (R2018b) 24-May-2018
 * C/C++ source code generated on : Wed Jul  7 12:07:11 2021
 *
 * Target selection: ert.tlc
 * Embedded hardware selection: Intel->x86-64 (Linux 64)
 * Code generation objectives: Unspecified
 * Validation result: Not run
 */

#include "ASM_modal_controller_S1.h"
#include "ASM_modal_controller_S1_private.h"

/* Block states (default storage) */
DW_ASM_modal_controller_S1_T ASM_modal_controller_S1_DW;

/* External inputs (root inport signals with default storage) */
ExtU_ASM_modal_controller_S1_T ASM_modal_controller_S1_U;

/* External outputs (root outports fed by signals with default storage) */
ExtY_ASM_modal_controller_S1_T ASM_modal_controller_S1_Y;

/* Real-time model */
RT_MODEL_ASM_modal_controller_S1_T ASM_modal_controller_S1_M_;
RT_MODEL_ASM_modal_controller_S1_T *const ASM_modal_controller_S1_M =
  &ASM_modal_controller_S1_M_;

/* Model step function */
void ASM_modal_controller_S1_step(void)
{
  int32_T j;
  real_T rtb_oldcmd[66];
  int32_T rtb_PulseGenerator;
  real_T rtb_deltacmd[66];
  real_T rtb_Polynomial;
  real_T rtb_Product2[66];
  real_T rtb_Product1[66];
  real_T rtb_Kb[66];
  int32_T i;
  real_T ASMPIcontroller_tmp[66];
  real_T rtb_deltacmd_j;
  real_T rtb_Product2_g;
  int_T idx;
  int_T Delay_DSTATE_tmp;
  for (i = 0; i < 66; i++) {
    /* Sum: '<S2>/Sum' incorporates:
     *  Delay: '<S2>/Delay'
     *  Inport: '<Root>/ASM_cmd'
     */
    rtb_deltacmd[i] = ASM_modal_controller_S1_U.ASM_cmd[i] -
      ASM_modal_controller_S1_DW.Delay_DSTATE[i];

    /* Delay: '<S2>/Delay' */
    rtb_oldcmd[i] = ASM_modal_controller_S1_DW.Delay_DSTATE[i];
  }

  /* DiscretePulseGenerator: '<S2>/Pulse Generator' */
  rtb_PulseGenerator = ((ASM_modal_controller_S1_DW.clockTickCounter < 1) &&
                        (ASM_modal_controller_S1_DW.clockTickCounter >= 0));
  if (ASM_modal_controller_S1_DW.clockTickCounter >= 9) {
    ASM_modal_controller_S1_DW.clockTickCounter = 0;
  } else {
    ASM_modal_controller_S1_DW.clockTickCounter++;
  }

  /* End of DiscretePulseGenerator: '<S2>/Pulse Generator' */

  /* DiscreteIntegrator: '<S2>/Discrete-Time Integrator' */
  if ((rtb_PulseGenerator > 0) &&
      (ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_PrevResetState <= 0)) {
    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE = 0.0;
  }

  /* Polyval: '<S2>/Polynomial2' incorporates:
   *  DiscreteIntegrator: '<S2>/Discrete-Time Integrator'
   */
  rtb_Polynomial = ((6.1843584E+18 *
                     ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE +
                     -4.2166079999999995E+15) *
                    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE +
                    6.3888E+11) *
    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE;

  /* Product: '<S2>/Product2' */
  for (i = 0; i < 66; i++) {
    rtb_Product2[i] = rtb_deltacmd[i] * rtb_Polynomial;
  }

  /* End of Product: '<S2>/Product2' */

  /* Polyval: '<S2>/Polynomial1' incorporates:
   *  DiscreteIntegrator: '<S2>/Discrete-Time Integrator'
   */
  rtb_Polynomial = ((1.5460896E+18 *
                     ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE +
                     -1.4055359999999998E+15) *
                    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE +
                    3.1944E+11) *
    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE *
    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE;

  /* Product: '<S2>/Product1' */
  for (i = 0; i < 66; i++) {
    rtb_Product1[i] = rtb_deltacmd[i] * rtb_Polynomial;
  }

  /* End of Product: '<S2>/Product1' */

  /* Polyval: '<S2>/Polynomial' incorporates:
   *  Delay: '<S2>/Delay'
   *  DiscreteIntegrator: '<S2>/Discrete-Time Integrator'
   *  DiscreteTransferFcn: '<S1>/ASM PI controller'
   *  DiscreteTransferFcn: '<S1>/ASM PI controller'
   *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
   *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
   *  Polyval: '<S2>/Polynomial1'
   *  Polyval: '<S2>/Polynomial2'
   */
  rtb_Polynomial = 3.0921792E+17;
  for (i = 0; i < 5; i++) {
    rtb_Polynomial = rtb_Polynomial *
      ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE +
      ASM_modal_controller_S1_ConstP.Polynomial_Coefs[i + 1];
  }

  /* Outputs for IfAction SubSystem: '<S2>/If Action Subsystem1' incorporates:
   *  ActionPort: '<S5>/Action Port'
   */
  for (i = 0; i < 66; i++) {
    /* If: '<S2>/If' incorporates:
     *  Constant: '<S3>/Constant'
     *  DiscreteIntegrator: '<S2>/Discrete-Time Integrator'
     *  Gain: '<S5>/Gain'
     *  Inport: '<S4>/In3'
     *  Inport: '<S5>/delta'
     *  Product: '<S2>/Product'
     *  RelationalOperator: '<S3>/Compare'
     *  SignalConversion: '<S5>/OutportBufferForOut2'
     */
    rtb_Product2_g = rtb_Product2[i];
    rtb_deltacmd_j = rtb_deltacmd[i];
    if (ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE <=
        0.00045454545454545455) {
      /* Outputs for IfAction SubSystem: '<S2>/If Action Subsystem' incorporates:
       *  ActionPort: '<S4>/Action Port'
       */
      rtb_deltacmd_j = rtb_deltacmd[i] * rtb_Polynomial;

      /* End of Outputs for SubSystem: '<S2>/If Action Subsystem' */
    } else {
      rtb_Product2_g = 0.0 * rtb_deltacmd[i];
      rtb_Product1[i] = rtb_Product2_g;
    }

    rtb_Product2[i] = rtb_Product2_g;

    /* Outputs for IfAction SubSystem: '<S2>/If Action Subsystem' incorporates:
     *  ActionPort: '<S4>/Action Port'
     */
    rtb_deltacmd[i] = rtb_deltacmd_j;

    /* End of If: '<S2>/If' */
    /* End of Outputs for SubSystem: '<S2>/If Action Subsystem' */
  }

  /* End of Outputs for SubSystem: '<S2>/If Action Subsystem1' */
  for (i = 0; i < 66; i++) {
    /* Sum: '<S2>/Add' */
    rtb_deltacmd_j = rtb_deltacmd[i] + rtb_oldcmd[i];

    /* Gain: '<S1>/Kb' incorporates:
     *  Sum: '<S2>/Add'
     */
    rtb_Kb[i] = 33.6 * rtb_Product1[i];

    /* DiscreteTransferFcn: '<S1>/ASM PI controller' incorporates:
     *  Inport: '<Root>/ASM_FB'
     *  Sum: '<S1>/Sum1'
     *  Sum: '<S2>/Add'
     */
    rtb_Polynomial = (rtb_deltacmd_j - ASM_modal_controller_S1_U.ASM_FB[i]) -
      (-ASM_modal_controller_S1_DW.ASMPIcontroller_states[i]);
    ASMPIcontroller_tmp[i] = rtb_Polynomial;
    rtb_Polynomial *= 70006.25321474488;
    rtb_Product1[i] = -69993.74678525512 *
      ASM_modal_controller_S1_DW.ASMPIcontroller_states[i] + rtb_Polynomial;

    /* Sum: '<S2>/Add' */
    rtb_deltacmd[i] = rtb_deltacmd_j;

    /* Gain: '<S1>/Km' incorporates:
     *  Sum: '<S2>/Add'
     */
    rtb_oldcmd[i] = 0.0112 * rtb_Product2[i];
  }

  for (i = 0; i < 66; i++) {
    /* DiscreteTransferFcn: '<S1>/Numerical differentiation' incorporates:
     *  Delay: '<S2>/Delay'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  Inport: '<Root>/ASM_FB'
     *  Polyval: '<S2>/Polynomial'
     *  Polyval: '<S2>/Polynomial1'
     *  Polyval: '<S2>/Polynomial2'
     */
    rtb_Polynomial = ASM_modal_controller_S1_U.ASM_FB[i] - -0.52169844279381428 *
      ASM_modal_controller_S1_DW.Numericaldifferentiation_states[i];
    rtb_Product2[i] = rtb_Polynomial;
    rtb_Polynomial *= 19122.2265954403;
    rtb_Polynomial += -19122.2265954403 *
      ASM_modal_controller_S1_DW.Numericaldifferentiation_states[i];

    /* Outport: '<Root>/ASM_modalF' incorporates:
     *  Delay: '<S2>/Delay'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  Gain: '<S1>/Kd'
     *  Gain: '<S1>/Ks'
     *  Polyval: '<S2>/Polynomial'
     *  Polyval: '<S2>/Polynomial1'
     *  Polyval: '<S2>/Polynomial2'
     *  Sum: '<S1>/Sum2'
     *  Sum: '<S1>/Sum3'
     *  Sum: '<S1>/Sum4'
     *  Sum: '<S1>/Sum5'
     */
    rtb_Product2_g = 0.0;
    idx = 0;
    for (j = 0; j < 66; j++) {
      rtb_Product2_g += ASM_modal_controller_S1_ConstP.Ks_Gain[idx + i] *
        rtb_deltacmd[j];
      idx += 66;
    }

    ASM_modal_controller_S1_Y.ASM_modalF[i] = ((rtb_oldcmd[i] + rtb_Kb[i]) +
      rtb_Product2_g) + (rtb_Product1[i] - 24.5 * rtb_Polynomial);

    /* End of Outport: '<Root>/ASM_modalF' */

    /* Outport: '<Root>/Mag_modal_vel' incorporates:
     *  Delay: '<S2>/Delay'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/ASM PI controller'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
     *  Polyval: '<S2>/Polynomial'
     *  Polyval: '<S2>/Polynomial1'
     *  Polyval: '<S2>/Polynomial2'
     */
    ASM_modal_controller_S1_Y.Mag_modal_vel[i] = rtb_Polynomial;
  }

  /* Update for Delay: '<S2>/Delay' incorporates:
   *  DiscreteTransferFcn: '<S1>/ASM PI controller'
   *  DiscreteTransferFcn: '<S1>/ASM PI controller'
   *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
   *  DiscreteTransferFcn: '<S1>/Numerical differentiation'
   *  Polyval: '<S2>/Polynomial'
   *  Polyval: '<S2>/Polynomial1'
   *  Polyval: '<S2>/Polynomial2'
   */
  idx = 0;
  for (i = 0; i < 9; i++) {
    for (j = 0; j < 66; j++) {
      Delay_DSTATE_tmp = j + idx;
      ASM_modal_controller_S1_DW.Delay_DSTATE[Delay_DSTATE_tmp] =
        ASM_modal_controller_S1_DW.Delay_DSTATE[Delay_DSTATE_tmp + 66];
    }

    idx += 66;
  }

  /* Update for DiscreteIntegrator: '<S2>/Discrete-Time Integrator' */
  ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_DSTATE += 2.5E-5;
  if (rtb_PulseGenerator > 0) {
    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_PrevResetState = 1;
  } else {
    ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_PrevResetState = 0;
  }

  /* End of Update for DiscreteIntegrator: '<S2>/Discrete-Time Integrator' */

  /* Update for Delay: '<S2>/Delay' incorporates:
   *  Inport: '<Root>/ASM_cmd'
   */
  memcpy(&ASM_modal_controller_S1_DW.Delay_DSTATE[594],
         &ASM_modal_controller_S1_U.ASM_cmd[0], 66U * sizeof(real_T));

  /* Update for DiscreteTransferFcn: '<S1>/ASM PI controller' incorporates:
   *  Delay: '<S2>/Delay'
   */
  memcpy(&ASM_modal_controller_S1_DW.ASMPIcontroller_states[0],
         &ASMPIcontroller_tmp[0], 66U * sizeof(real_T));

  /* Update for DiscreteTransferFcn: '<S1>/Numerical differentiation' incorporates:
   *  Delay: '<S2>/Delay'
   */
  memcpy(&ASM_modal_controller_S1_DW.Numericaldifferentiation_states[0],
         &rtb_Product2[0], 66U * sizeof(real_T));
}

/* Model initialize function */
void ASM_modal_controller_S1_initialize(void)
{
  /* Registration code */

  /* initialize error status */
  rtmSetErrorStatus(ASM_modal_controller_S1_M, (NULL));

  /* states (dwork) */
  (void) memset((void *)&ASM_modal_controller_S1_DW, 0,
                sizeof(DW_ASM_modal_controller_S1_T));

  /* external inputs */
  (void)memset(&ASM_modal_controller_S1_U, 0, sizeof
               (ExtU_ASM_modal_controller_S1_T));

  /* external outputs */
  (void) memset((void *)&ASM_modal_controller_S1_Y, 0,
                sizeof(ExtY_ASM_modal_controller_S1_T));

  /* InitializeConditions for DiscretePulseGenerator: '<S2>/Pulse Generator' */
  ASM_modal_controller_S1_DW.clockTickCounter = 0;

  /* InitializeConditions for DiscreteIntegrator: '<S2>/Discrete-Time Integrator' */
  ASM_modal_controller_S1_DW.DiscreteTimeIntegrator_PrevResetState = 2;
}

/* Model terminate function */
void ASM_modal_controller_S1_terminate(void)
{
  /* (no terminate code required) */
}

/*
 * File trailer for generated code.
 *
 * [EOF]
 */
