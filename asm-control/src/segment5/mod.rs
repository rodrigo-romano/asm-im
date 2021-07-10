use dosio::{io::Tags, ios, DOSIOSError, Dos, IOTags, IO};
use simulink_rs::*;

import_simulink!(ASM_modal_controller_S5, U : (ASM_cmd,66,ASM_FB,66), Y : (ASM_modalF,66,Mag_modal_vel,66));
build_inputs!(ASMCmd, 66, ASMFB, 66);
build_outputs!(ASMModalF, 66, MagModalVel, 66);
build_controller!(ASM_modal_controller_S5,
          U : (ASM_cmd -> (ASMCmd, asm_cmd),
               ASM_FB -> (ASMFB, asm_fb)),
          Y : (ASM_modalF -> (ASMModalF, asm_modal_f),
               Mag_modal_vel -> (MagModalVel, mag_modal_vel))
);
impl<'a> Controller<'a> {
    fn dispatch(pipe: &mut U<'a>, data: &[f64]) {
        data.iter().enumerate().for_each(|(k, &v)| {
            pipe[k] = v;
        });
    }
}

impl<'a> IOTags for Controller<'a> {
    fn outputs_tags(&self) -> Vec<Tags> {
        ios!(ASMCmd, M2S5FSRBModalD)
    }
    fn inputs_tags(&self) -> Vec<Tags> {
        ios!(M2S5FSCPModalF, M2S5FSRBModalF)
    }
}

impl<'a> Dos for Controller<'a> {
    type Input = Vec<f64>;
    type Output = Vec<f64>;
    fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError> {
        if let Some(data) = data {
            match (&data[ios!(M2S5Cmd)], &data[ios!(M2S5FSRBModalD)]) {
                (
                    IO::M2S5Cmd {
                        data: Some(asm_cmd),
                    },
                    IO::M2S5FSRBModalD { data: Some(asm_fb) },
                ) => {
                    Self::dispatch(&mut self.asm_cmd, asm_cmd);
                    Self::dispatch(&mut self.asm_fb, asm_fb);
                    Ok(self)
                }
                _ => Err(DOSIOSError::Inputs(
                    "Missing IO::ASMCmd and IO::M2S5FSRBModalD as inputs to ASM controller".into(),
                )),
            }
        } else {
            Err(DOSIOSError::Inputs(
                "Empty inputs passed to ASM controller".into(),
            ))
        }
    }
    fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>> {
        Some(vec![
            IO::M2S5FSCPModalF {
                data: Some(Vec::<f64>::from(&self.asm_modal_f)),
            },
            IO::M2S5FSRBModalF {
                data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
            },
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_test() {
        let mut asm_ctrl = Controller::new();
        let u = ios!(ASMCmd(vec![0f64; 66]), ASMFB(vec![0f64; 66]));
        let y = asm_ctrl.in_step_out(Some(u)).unwrap();
        println!("ASM MOUNT CONTROL ZEROS TEST: {:#?}", y);
    }
    #[test]
    fn cmd_piston_test() {
        let mut asm_ctrl = Controller::new();
        println!("ASM MOUNT CONTROL PISTON TEST",);
        for k in 0..10 {
            let mut asm_cmd = vec![0f64; 66];
            asm_cmd[0] = 1e-6;
            let u = ios!(M2S5FSRBModalD(vec![0f64; 66]), ASMCmd(asm_cmd));
            let y = asm_ctrl.in_step_out(Some(u)).unwrap().unwrap();
            println!(
                " {}: {:#?}",
                k,
                Option::<Vec<f64>>::from(&y[ios!(M2S5FSCPModalF)]).unwrap()[0]
            );
        }
    }

    #[test]
    fn fdb_piston_test() {
        let mut asm_ctrl = Controller::new(Segment::One);
        println!("ASM MOUNT CONTROL PISTON TEST",);
        for k in 0..10 {
            let mut asm_cmd = vec![0f64; 66];
            asm_cmd[0] = 1e-6;
            let u = ios!(M2S5FSRBModalD(asm_cmd), ASMCmd(vec![0f64; 66]));
            let y = asm_ctrl.in_step_out(Some(u)).unwrap().unwrap();
            println!(
                " {}: {:#?}",
                k,
                Option::<Vec<f64>>::from(&y[ios!(M2S5FSCPModalF)]).unwrap()[0]
            );
        }
    }
}
