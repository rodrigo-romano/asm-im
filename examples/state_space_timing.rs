use asm_control::ASMS;
use dosio::{ios, Dos, IOTags};
use fem::dos;
use simple_logger::SimpleLogger;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init()?;

    // ASM controller
    let mut asms = ASMS::new();

    let snd_ord = {
        let snd_ord = dos::SecondOrder::from_pickle("/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder_1500Hz_noRes_postproc_v1.pkl").unwrap();
        snd_ord.into(
            // Segment #1
            asms.inputs_tags(),
            {
                let mut u = asms.outputs_tags();
                u.append(
                    &mut (ios!(
                        /*M2S1FSModalD,
                        M2S2FSModalD,
                        M2S3FSModalD,
                        M2S4FSModalD,
                        M2S5FSModalD,
                        M2S6FSModalD,
                        M2S7FSModalD*/
                        M2segment1axiald,
                        M2segment2axiald,
                        M2segment3axiald,
                        M2segment4axiald,
                        M2segment5axiald,
                        M2segment6axiald,
                        M2segment7axiald
                    ))
                    .clone(),
                );
                u
            },
        )
    };
    println!("{}", &snd_ord);

    let sampling_rate = 8e3;
    let mut dms: dos::DiscreteModalSolver<dos::Exponential> = (snd_ord, sampling_rate).into();

    let n_step = 1000;
    /*let ss = &mut dms.state_space[0];
    let mut u = vec![0f64; ss.n_inputs()];
    u[0] = 1e-6;*/

    let mut asm_cmd = vec![0f64; 66];
    asm_cmd[0] = 1e-6;
    let mut u = ios!(
        M2S1FSRBModalD(vec![0f64; 66]),
        M2S2FSRBModalD(vec![0f64; 66]),
        M2S3FSRBModalD(vec![0f64; 66]),
        M2S4FSRBModalD(vec![0f64; 66]),
        M2S5FSRBModalD(vec![0f64; 66]),
        M2S6FSRBModalD(vec![0f64; 66]),
        M2S7FSRBModalD(vec![0f64; 66])
    );
    u.append(&mut ios!(
        M2S1Cmd(asm_cmd.clone()),
        M2S2Cmd(asm_cmd.clone()),
        M2S3Cmd(asm_cmd.clone()),
        M2S4Cmd(asm_cmd.clone()),
        M2S5Cmd(asm_cmd.clone()),
        M2S6Cmd(asm_cmd.clone()),
        M2S7Cmd(asm_cmd.clone())
    ));
    let y = asms.in_step_out(Some(u))?;

    let now = Instant::now();
    //dms.inputs(y.clone())?;
    //dms.step()?;
    for _ in 0..n_step {
        //ss.solve(&u);
        //let fem = dms.outputs();
        let fem = dms.in_step_out(y.clone())?;
    }
    let elapsed = now.elapsed().as_secs_f64();
    println!("Average time per step: {}ms", 1e3 * elapsed / n_step as f64);

    Ok(())
}
