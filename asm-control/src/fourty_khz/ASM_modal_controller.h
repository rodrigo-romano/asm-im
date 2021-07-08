#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
/*
 * File: ASM_modal_controller.h
 *
 * Code generated for Simulink model 'ASM_modal_controller'.
 *
 * Model version                  : 1.1283
 * Simulink Coder version         : 9.0 (R2018b) 24-May-2018
 * C/C++ source code generated on : Wed Jul  7 14:49:21 2021
 *
 * Target selection: ert.tlc
 * Embedded hardware selection: Intel->x86-64 (Linux 64)
 * Code generation objectives: Unspecified
 * Validation result: Not run
 */

#ifndef RTW_HEADER_ASM_modal_controller_h_
#define RTW_HEADER_ASM_modal_controller_h_
#include <string.h>
#include <stddef.h>
#ifndef ASM_modal_controller_COMMON_INCLUDES_
# define ASM_modal_controller_COMMON_INCLUDES_
#include "rtwtypes.h"
#endif                                 /* ASM_modal_controller_COMMON_INCLUDES_ */

#include "ASM_modal_controller_types.h"

/* Macros for accessing real-time model data structure */
#ifndef rtmGetErrorStatus
# define rtmGetErrorStatus(rtm)        ((rtm)->errorStatus)
#endif

#ifndef rtmSetErrorStatus
# define rtmSetErrorStatus(rtm, val)   ((rtm)->errorStatus = (val))
#endif

/* Block states (default storage) for system '<Root>' */
typedef struct {
  real_T Delay_DSTATE[660];            /* '<S3>/Delay' */
  real_T DiscreteTimeIntegrator_DSTATE;/* '<S3>/Discrete-Time Integrator' */
  real_T Numericaldifferentiation_states[66];/* '<S1>/Numerical differentiation' */
  real_T ASMPIcontroller_states[66];   /* '<S1>/ASM PI controller' */
  int32_T clockTickCounter;            /* '<S3>/Pulse Generator' */
  int8_T DiscreteTimeIntegrator_PrevResetState;/* '<S3>/Discrete-Time Integrator' */
} DW_ASM_modal_controller_T;

/* Constant parameters (default storage) */
typedef struct {
  /* Expression: [sm.d3SF/20, sm.d2SF/12, sm.d1SF/6, 0, 0, 0]
   * Referenced by: '<S3>/Polynomial'
   */
  real_T Polynomial_Coefs[6];

  /* Expression: Ks{1}
   * Referenced by: '<S1>/Ks'
   */
  real_T Ks_Gain[4356];
} ConstP_ASM_modal_controller_T;

/* External inputs (root inport signals with default storage) */
typedef struct {
  real_T ASM_cmd[66];                  /* '<Root>/ASM_cmd' */
  real_T ASM_FB[66];                   /* '<Root>/ASM_FB' */
} ExtU_ASM_modal_controller_T;

/* External outputs (root outports fed by signals with default storage) */
typedef struct {
  real_T ASM_modalF[66];               /* '<Root>/ASM_modalF' */
  real_T Mag_modal_vel[66];            /* '<Root>/Mag_modal_vel' */
} ExtY_ASM_modal_controller_T;

/* Real-time Model Data Structure */
struct tag_RTM_ASM_modal_controller_T {
  const char_T * volatile errorStatus;
};

/* Block states (default storage) */
extern DW_ASM_modal_controller_T ASM_modal_controller_DW;

/* External inputs (root inport signals with default storage) */
extern ExtU_ASM_modal_controller_T ASM_modal_controller_U;

/* External outputs (root outports fed by signals with default storage) */
extern ExtY_ASM_modal_controller_T ASM_modal_controller_Y;

/* Constant parameters (default storage) */
extern const ConstP_ASM_modal_controller_T ASM_modal_controller_ConstP;

/* Model entry point functions */
extern void ASM_modal_controller_initialize(void);
extern void ASM_modal_controller_step(void);
extern void ASM_modal_controller_terminate(void);

/* Real-time Model object */
extern RT_MODEL_ASM_modal_controller_T *const ASM_modal_controller_M;

/*-
 * These blocks were eliminated from the model due to optimizations:
 *
 * Block '<S1>/m2_asm_ff_en' : Eliminated nontunable gain of 1
 */

/*-
 * The generated code includes comments that allow you to trace directly
 * back to the appropriate location in the model.  The basic format
 * is <system>/block_name, where system is the system number (uniquely
 * assigned by Simulink) and block_name is the name of the block.
 *
 * Note that this particular code originates from a subsystem build,
 * and has its own system numbers different from the parent model.
 * Refer to the system hierarchy for this subsystem below, and use the
 * MATLAB hilite_system command to trace the generated code back
 * to the parent model.  For example,
 *
 * hilite_system('flex_asm_TE_v2b_rco/ASM_modal_controller')    - opens subsystem flex_asm_TE_v2b_rco/ASM_modal_controller
 * hilite_system('flex_asm_TE_v2b_rco/ASM_modal_controller/Kp') - opens and selects block Kp
 *
 * Here is the system hierarchy for this model
 *
 * '<Root>' : 'flex_asm_TE_v2b_rco'
 * '<S1>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller'
 * '<S2>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller/Pre shape + FF terms'
 * '<S3>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller/Pre shape + FF terms/Cmd Shaping Filter'
 * '<S4>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller/Pre shape + FF terms/Command  Shaping Filter'
 * '<S5>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller/Pre shape + FF terms/Cmd Shaping Filter/Compare To Constant'
 * '<S6>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller/Pre shape + FF terms/Cmd Shaping Filter/If Action Subsystem'
 * '<S7>'   : 'flex_asm_TE_v2b_rco/ASM_modal_controller/Pre shape + FF terms/Cmd Shaping Filter/If Action Subsystem1'
 */
#endif                                 /* RTW_HEADER_ASM_modal_controller_h_ */

/*
 * File trailer for generated code.
 *
 * [EOF]
 */
