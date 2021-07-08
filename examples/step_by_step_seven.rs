use asm_control;
use dosio::{io::jar, Dos, IO};
use fem::dos;

fn main() {
    let asm_inputs = vec![
        jar::M2S1FSCPModalF::io::<()>(),
        jar::M2S1FSRBModalF::io::<()>(),
        jar::M2S2FSCPModalF::io::<()>(),
        jar::M2S2FSRBModalF::io::<()>(),
        jar::M2S3FSCPModalF::io::<()>(),
        jar::M2S3FSRBModalF::io::<()>(),
        jar::M2S4FSCPModalF::io::<()>(),
        jar::M2S4FSRBModalF::io::<()>(),
        jar::M2S5FSCPModalF::io::<()>(),
        jar::M2S5FSRBModalF::io::<()>(),
        jar::M2S6FSCPModalF::io::<()>(),
        jar::M2S6FSRBModalF::io::<()>(),
        jar::M2S7FSCPModalF::io::<()>(),
        jar::M2S7FSRBModalF::io::<()>(),
    ];
    let asm_outputs = vec![
        jar::M2S1FSRBModalD::io::<()>(),
        jar::M2S1FSModalD::io::<()>(),
        jar::M2S2FSRBModalD::io::<()>(),
        jar::M2S2FSModalD::io::<()>(),
        jar::M2S3FSRBModalD::io::<()>(),
        jar::M2S3FSModalD::io::<()>(),
        jar::M2S4FSRBModalD::io::<()>(),
        jar::M2S4FSModalD::io::<()>(),
        jar::M2S5FSRBModalD::io::<()>(),
        jar::M2S5FSModalD::io::<()>(),
        jar::M2S6FSRBModalD::io::<()>(),
        jar::M2S6FSModalD::io::<()>(),
        jar::M2S7FSRBModalD::io::<()>(),
        jar::M2S7FSRBModalF::io::<()>(),
    ];

    let snd_ord = {
        let snd_ord = dos::SecondOrder::from_pickle("/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder_1500Hz_noRes_postproc.pkl").unwrap();
        snd_ord.into(asm_inputs, asm_outputs)
    };
    println!("{}", &snd_ord);
    let sampling_rate = 8e3;
    let mut dms: dos::DiscreteModalSolver<dos::Exponential> = (snd_ord, sampling_rate).into();

    let segments = vec![
        asm_control::Segment::One,
        asm_control::Segment::Two,
        asm_control::Segment::Three,
        asm_control::Segment::Four,
        asm_control::Segment::Five,
        asm_control::Segment::Six,
        asm_control::Segment::Seven,
    ];
    let mut asm_ctrlrs: Vec<_> = segments
        .into_iter()
        .map(asm_control::Controller::new)
        .collect();
    let fluid_damping_gain = -9.1_f64;

    let mut fem: Option<Vec<IO<Vec<f64>>>> = None;

    let mut asm_cmd = vec![0f64; 66];
    asm_cmd[0] = 1e-6;

    let mut y = vec![
        jar::M2S1FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S1FSRBModalF::io_with(vec![0f64; 66]),
        jar::M2S2FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S2FSRBModalF::io_with(vec![0f64; 66]),
        jar::M2S3FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S3FSRBModalF::io_with(vec![0f64; 66]),
        jar::M2S4FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S4FSRBModalF::io_with(vec![0f64; 66]),
        jar::M2S5FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S5FSRBModalF::io_with(vec![0f64; 66]),
        jar::M2S6FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S6FSRBModalF::io_with(vec![0f64; 66]),
        jar::M2S7FSCPModalF::io_with(vec![0f64; 66]),
        jar::M2S7FSRBModalF::io_with(vec![0f64; 66]),
    ];

    for k in 0..20 {
        fem = dms.in_step_out(Some(y)).unwrap();
        if let Some(ref x) = fem {
            print!(
                "#{:2}   {:7.4}  {:7.4}",
                k,
                1e6 * Option::<Vec<f64>>::from(&x[jar::M2S1FSModalD::io::<()>()]).unwrap()[0],
                1e6 * Option::<Vec<f64>>::from(&x[jar::M2S1FSRBModalD::io::<()>()]).unwrap()[0],
            );
        };

        let u = match fem {
            Some(values) => vec![
                vec![
                    values[jar::M2S1FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    values[jar::M2S2FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    values[jar::M2S3FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    values[jar::M2S4FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    values[jar::M2S5FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    values[jar::M2S6FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    values[jar::M2S7FSRBModalD::io::<()>()].clone(),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
            ],
            None => vec![
                vec![
                    jar::M2S1FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    jar::M2S2FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    jar::M2S3FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    jar::M2S4FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    jar::M2S5FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    jar::M2S6FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
                vec![
                    jar::M2S7FSRBModalD::io_with(vec![0f64; 66]),
                    jar::ASMCmd::io_with(asm_cmd.clone()),
                ],
            ],
        };
        let velocities = vec![
            jar::M2S1FSRBModalF::io::<()>(),
            jar::M2S2FSRBModalF::io::<()>(),
            jar::M2S3FSRBModalF::io::<()>(),
            jar::M2S4FSRBModalF::io::<()>(),
            jar::M2S5FSRBModalF::io::<()>(),
            jar::M2S6FSRBModalF::io::<()>(),
            jar::M2S7FSRBModalF::io::<()>(),
        ];
        y = asm_ctrlrs
            .iter_mut()
            .zip(u.into_iter().zip(velocities.into_iter()))
            .filter_map(|(asm_ctrl, (u, vel))| {
                let mut y = asm_ctrl.in_step_out(Some(u)).unwrap();
                if let Some(ref mut v) = y {
                    v[vel] *= fluid_damping_gain;
                }
                /*
                        println!(
                            "  {:+6.4}  {:+6.4}",
                            Option::<Vec<f64>>::from(&y.clone().unwrap()[jar::M2S1FSCPModalF::io::<()>()])
                                .unwrap()[0],
                            Option::<Vec<f64>>::from(&y.clone().unwrap()[jar::M2S1FSRBModalF::io::<()>()])
                                .unwrap()[0],
                        );
                */
                y
            })
            .flatten()
            .collect();
        println!();
    }
}
