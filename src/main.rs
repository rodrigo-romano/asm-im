use asm_control::ASMS;
use colorous;
use dosio::{ios, Dos, IOTags, IO};
use fem::dos;
use plotters::prelude::*;
use std::time::Instant;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "ASM-IM", about = "Integrated Model with FDR ASM")]
struct Opt {
    /// # of steps
    n_step: usize,
    /// Modal forces gain
    #[structopt(short = "g", long = "gain", default_value = "0.5")]
    modal_forces_gain: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // ASM controller
    let logs_data_tags = ios!(
        M2S1FSModalD,
        M2S2FSModalD,
        M2S3FSModalD,
        M2S4FSModalD,
        M2S5FSModalD,
        M2S6FSModalD,
        M2S7FSModalD
    );
    let mut asms = ASMS::new().modal_forces_gain(opt.modal_forces_gain);
    // ASM command input (segment #1)
    let mut asm_cmd = vec![0f64; 66];
    asm_cmd[0] = 1e-6;

    // Finite element model
    let snd_ord = {
        let snd_ord = dos::SecondOrder::from_pickle("/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder_1500Hz_noRes_postproc.pkl")?;
        // Model reduction
        snd_ord.into(
            // Segment #1
            asms.inputs_tags(),
            {
                let mut u = asms.outputs_tags();
                u.append(&mut logs_data_tags.clone());
                u
            },
        )
    };
    println!("{}", &snd_ord);
    // Discrete state space model from 2nd order FEM
    let sampling_rate = 8e3;
    let mut dms: dos::DiscreteModalSolver<dos::Exponential> = (snd_ord, sampling_rate).into();

    // FEM state space output
    let mut fem: Option<Vec<IO<Vec<f64>>>> = None;

    let n_step = opt.n_step;
    let decimation = 1_usize;
    let mut modal_fs: Vec<IO<Vec<f64>>> =
        Vec::with_capacity(logs_data_tags.len() * n_step / decimation);
    let now = Instant::now();
    for _k in 0..n_step {
        let mut u = match fem {
            Some(x) => x,
            None => ios!(
                M2S1FSRBModalD(vec![0f64; 66]),
                M2S2FSRBModalD(vec![0f64; 66]),
                M2S3FSRBModalD(vec![0f64; 66]),
                M2S4FSRBModalD(vec![0f64; 66]),
                M2S5FSRBModalD(vec![0f64; 66]),
                M2S6FSRBModalD(vec![0f64; 66]),
                M2S7FSRBModalD(vec![0f64; 66])
            ),
        };
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

        #[cfg(debug)]
        print!(
            "#{:2}   {:+6.4}  {:+6.4}",
            k,
            Option::<Vec<f64>>::from(&y.clone().unwrap()[ios!(M2S1FSCPModalF)]).unwrap()[0],
            Option::<Vec<f64>>::from(&y.clone().unwrap()[ios!(M2S1FSRBModalF)]).unwrap()[0],
        );

        // FEM update & output:
        fem = dms.in_step_out(y)?;
        //  - logging the face sheet outputs
        if let Some(ref x) = &fem {
            logs_data_tags.iter().for_each(|t| {
                modal_fs.push(x[t].clone());
            });
            /*if k % decimation == 0 {
                    modal_fs.push(x[jar::M2S1FSModalD::io::<()>()].clone());
            }*/
            #[cfg(debug)]
            println!(
                "  {:7.4}  {:7.4}",
                1e6 * Option::<Vec<f64>>::from(&x[ios!(M2S1FSModalD)]).unwrap()[0],
                1e6 * Option::<Vec<f64>>::from(&x[ios!(M2S1FSRBModalD)]).unwrap()[0],
            );
        };
    }
    let elapsed = now.elapsed().as_millis();
    println!(
        "Elapsed time: {}ms [{:.3}ms/step]",
        elapsed,
        elapsed as f64 / n_step as f64
    );

    // Post-processing logged data
    let n_logs = logs_data_tags.len();
    let (segment_piston, segment_piston_minmax): (Vec<_>, Vec<_>) = (0..n_logs)
        .map(|i| {
            let time_series: Vec<_> = modal_fs
                .iter()
                .skip(i)
                .step_by(n_logs)
                .map(|y| Option::<Vec<f64>>::from(y).unwrap())
                .collect();
            let piston: Vec<_> = time_series.iter().map(|x| x[0] * 1e6).collect();
            let piston_max = piston.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let piston_min = piston.iter().cloned().fold(f64::INFINITY, f64::min);
            println!("Piston min/max: [{},{}]", piston_min, piston_max);
            (piston, (piston_min, piston_max))
        })
        .unzip();

    let root_area = BitMapBackend::new("modal_facesheet.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE)?;

    // Plotting ...
    let piston_max = segment_piston_minmax
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, |m, (_, p)| f64::max(m, p));
    let piston_min = segment_piston_minmax
        .into_iter()
        .fold(f64::INFINITY, |m, (p, _)| f64::min(m, p));
    let mut ctx = ChartBuilder::on(&root_area)
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        //.caption("Line Plot Demo", ("sans-serif", 40))
        //.build_cartesian_2d(0f64..n_step as f64 / sampling_rate, -1e-3f64..10e-3f64)
        .build_cartesian_2d(0f64..n_step as f64 / sampling_rate, piston_min..piston_max)?;

    ctx.configure_mesh()
        .x_desc("Time [s]")
        .y_desc("Piston")
        .draw()?;

    let mut colors = colorous::TABLEAU10.iter().cycle();

    for piston in segment_piston.iter() {
        let color = colors.next().unwrap().as_tuple();
        let rgb = RGBColor(color.0, color.1, color.2);
        ctx.draw_series(LineSeries::new(
            piston
                .iter()
                .enumerate()
                .map(|(k, &x)| (k as f64 / sampling_rate, x)),
            &rgb,
        ))?;
    }
    Ok(())
}
