use dosio::{
    io::{jar, Tags},
    DOSIOSError, Dos, IOTags, IO,
};
use simulink_rs::*;

import_simulink!(ASM_modal_controller_S1, U : (ASM_cmd,66,ASM_FB,66), Y : (ASM_modalF,66,Mag_modal_vel,66));
build_inputs!(ASMCmd, 66, ASMFB, 66);
build_outputs!(ASMModalF, 66, MagModalVel, 66);
build_controller_with_data!(ASM_modal_controller_S1,
          U : (ASM_cmd -> (ASMCmd, asm_cmd),
               ASM_FB -> (ASMFB, asm_fb)),
          Y : (ASM_modalF -> (ASMModalF, asm_modal_f),
               Mag_modal_vel -> (MagModalVel, mag_modal_vel))
);
impl<'a, T> Controller<'a, T> {
    fn dispatch(pipe: &mut U<'a>, data: &[f64]) {
        data.iter().enumerate().for_each(|(k, &v)| {
            pipe[k] = v;
        });
    }
}

pub enum Segment {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl<'a, T> IOTags for Controller<'a, T> {
    fn outputs_tags(&self) -> Vec<Tags> {
        vec![jar::ASMCmd::io(), jar::ASMFB::io()]
    }
    fn inputs_tags(&self) -> Vec<Tags> {
        vec![jar::ASMModalF::io(), jar::MagModalVel::io()]
    }
}

impl<'a> Dos for Controller<'a, Segment> {
    type Input = Vec<f64>;
    type Output = Vec<f64>;
    fn inputs(&mut self, data: Option<Vec<IO<Self::Input>>>) -> Result<&mut Self, DOSIOSError> {
        if let Some(data) = data {
            match self.data {
                Segment::One => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S1FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S1FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
                Segment::Two => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S2FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S2FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
                Segment::Three => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S3FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S3FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
                Segment::Four => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S4FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S4FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
                Segment::Five => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S5FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S5FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
                Segment::Six => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S6FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S6FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
                Segment::Seven => match (
                    &data[jar::ASMCmd::io::<()>()],
                    &data[jar::M2S7FSRBModalD::io::<()>()],
                ) {
                    (
                        IO::ASMCmd {
                            data: Some(asm_cmd),
                        },
                        IO::M2S7FSRBModalD { data: Some(asm_fb) },
                    ) => {
                        Self::dispatch(&mut self.asm_cmd, asm_cmd);
                        Self::dispatch(&mut self.asm_fb, asm_fb);
                        Ok(self)
                    }
                    _ => Err(DOSIOSError::Inputs(
                        "Missing IO::ASMCmd and IO::ASMFB as inputs to ASM controller".into(),
                    )),
                },
            }
        } else {
            Err(DOSIOSError::Inputs(
                "Empty inputs passed to ASM controller".into(),
            ))
        }
    }
    fn outputs(&mut self) -> Option<Vec<IO<Self::Output>>> {
        Some(match self.data {
            Segment::One => {
                vec![
                    IO::M2S1FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S1FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
            Segment::Two => {
                vec![
                    IO::M2S2FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S2FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
            Segment::Three => {
                vec![
                    IO::M2S3FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S3FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
            Segment::Four => {
                vec![
                    IO::M2S4FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S4FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
            Segment::Five => {
                vec![
                    IO::M2S5FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S5FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
            Segment::Six => {
                vec![
                    IO::M2S6FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S6FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
            Segment::Seven => {
                vec![
                    IO::M2S7FSCPModalF {
                        data: Some(Vec::<f64>::from(&self.asm_modal_f)),
                    },
                    IO::M2S7FSRBModalF {
                        data: Some(Vec::<f64>::from(&self.mag_modal_vel)),
                    },
                ]
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeros_test() {
        let mut asm_ctrl = Controller::new(Segment::One);
        let u = vec![
            jar::ASMCmd::io_with(vec![0f64; 66]),
            jar::ASMFB::io_with(vec![0f64; 66]),
        ];
        let y = asm_ctrl.in_step_out(Some(u)).unwrap();
        println!("ASM MOUNT CONTROL ZEROS TEST: {:#?}", y);
    }
    #[test]
    fn cmd_piston_test() {
        let mut asm_ctrl = Controller::new(Segment::One);
        println!("ASM MOUNT CONTROL PISTON TEST",);
        for k in 0..10 {
            let mut asm_cmd = vec![0f64; 66];
            asm_cmd[0] = 1e-6;
            let u = vec![
                jar::M2S1FSRBModalD::io_with(vec![0f64; 66]),
                jar::ASMCmd::io_with(asm_cmd),
            ];
            let y = asm_ctrl.in_step_out(Some(u)).unwrap().unwrap();
            println!(
                " {}: {:#?}",
                k,
                Option::<Vec<f64>>::from(&y[jar::M2S1FSCPModalF::io::<()>()]).unwrap()[0]
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
            let u = vec![
                jar::M2S1FSRBModalD::io_with(asm_cmd),
                jar::ASMCmd::io_with(vec![0f64; 66]),
            ];
            let y = asm_ctrl.in_step_out(Some(u)).unwrap().unwrap();
            println!(
                " {}: {:#?}",
                k,
                Option::<Vec<f64>>::from(&y[jar::M2S1FSCPModalF::io::<()>()]).unwrap()[0]
            );
        }
    }
}
