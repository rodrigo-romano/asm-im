use dosio::{ios, Dos};
use fem::dos;
use simple_logger::SimpleLogger;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    SimpleLogger::new().init()?;

    let snd_ord = {
        let snd_ord = dos::SecondOrder::from_pickle("/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder_1500Hz_noRes_postproc.pkl").unwrap();
        println!("{}", snd_ord);
        println!(
            "B sum: {}",
            snd_ord.b_rows().map(|r| r.iter().sum::<f64>()).sum::<f64>()
        );
        println!(
            "C sum: {}",
            snd_ord.c_rows().map(|r| r.iter().sum::<f64>()).sum::<f64>()
        );
        snd_ord.into(
            ios!(
                M2S1FSCPModalF,
                M2S1FSRBModalF,
                M2S2FSCPModalF,
                M2S2FSRBModalF
            ),
            ios!(M2S1FSRBModalD, M2S1FSModalD, M2S2FSRBModalD, M2S2FSModalD),
        )
    };
    println!("{}", &snd_ord);
    println!(
        "B sum: {}",
        snd_ord.b_rows().map(|r| r.iter().sum::<f64>()).sum::<f64>()
    );
    println!(
        "C sum: {}",
        snd_ord.c_rows().map(|r| r.iter().sum::<f64>()).sum::<f64>()
    );

    let sampling_rate = 8e3;
    let mut dms: dos::DiscreteModalSolver<dos::Exponential> = (snd_ord, sampling_rate).into();
    println!("{}", dms.state_space[0]);
    println!("{}", dms.state_space.last().unwrap());

    /*
       let ss = &mut dms.state_space[0];
       let mut u = vec![0f64; ss.n_inputs()];
       u[0] = 1e-6;
       for _ in 0..10 {
           println!("Solve 0: {}", ss.solve(&u).iter().sum::<f64>());
       }
    */

    let mut asm_cmd = vec![0f64; 66];
    //asm_cmd[0] = 1e-6;
    asm_cmd[0] = 0.389204179097122;
    let asm = Some(ios!(
        M2S1FSCPModalF(asm_cmd),
        M2S1FSRBModalF(vec![0f64; 66]),
        M2S2FSCPModalF(vec![0f64; 66]),
        M2S2FSRBModalF(vec![0f64; 66])
    ));

    let now = Instant::now();
    let n_step = 20_u128;
    for k in 0..n_step {
        //println!("{:#?}", asm);
        if let Some(x) = dms.in_step_out(asm.clone())? {
            let y10 = Option::<Vec<f64>>::from(&x[ios!(M2S1FSModalD)]).unwrap();
            let y11 = Option::<Vec<f64>>::from(&x[ios!(M2S1FSRBModalD)]).unwrap();
            let y20 = Option::<Vec<f64>>::from(&x[ios!(M2S2FSModalD)]).unwrap();
            let y21 = Option::<Vec<f64>>::from(&x[ios!(M2S2FSRBModalD)]).unwrap();
            println!(
                "#{:2}: {:8.4}, {:8.4} {:8.4}, {:8.4} {:8.4}, {:8.4}",
                k,
                1e6 * y10[0],
                1e6 * y11[0],
                1e6 * y10.iter().sum::<f64>(),
                1e6 * y11.iter().sum::<f64>(),
                1e6 * y20[0],
                1e6 * y21[0],
            );
        };
    }
    let elapsed = now.elapsed().as_millis();
    println!(
        "Elapsed time: {}ms [{:.3}ms/step]",
        elapsed,
        elapsed as f64 / n_step as f64
    );

    Ok(())
}
