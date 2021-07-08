use colorous;
use dosio::{io::jar, Dos, IO};
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

    let snd_ord = {
        let snd_ord = dos::SecondOrder::from_pickle("/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder_1500Hz_noRes_postproc.pkl")?;
        snd_ord.into(
            vec![jar::M2S1FSCPModalF::io(), jar::M2S1FSRBModalF::io()],
            vec![jar::M2S1FSRBModalD::io(), jar::M2S1FSModalD::io()],
        )
    };
    println!("{}", &snd_ord);
    let sampling_rate = 8e3;
    let mut dms: dos::DiscreteModalSolver<dos::Exponential> = (snd_ord, sampling_rate).into();

    let mut asm_ctrl = asm_control::Controller::new(asm_control::Segment::One);
    let fluid_damping_gain = -9.1_f64;
    let mut fem: Option<Vec<IO<Vec<f64>>>> = None;

    let mut asm_cmd = vec![0f64; 66];
    asm_cmd[0] = 1e-6;

    let n_step = opt.n_step;
    let decimation = 1_usize;
    let mut modal_fs: Vec<IO<Vec<f64>>> = Vec::with_capacity(n_step / decimation);

    let now = Instant::now();
    for _k in 0..n_step {
        let u = match fem {
            Some(values) => vec![
                //jar::M2S1FSRBModalD::io_with(vec![0f64; 66]),
                values[jar::M2S1FSRBModalD::io::<()>()].clone(),
                jar::ASMCmd::io_with(asm_cmd.clone()),
            ],
            None => vec![
                jar::M2S1FSRBModalD::io_with(vec![0f64; 66]),
                jar::ASMCmd::io_with(asm_cmd.clone()),
            ],
        };
        let mut y = asm_ctrl.in_step_out(Some(u.clone()))?;
        if let Some(ref mut v) = y {
            v[jar::M2S1FSCPModalF::io::<()>()] *= opt.modal_forces_gain;
            v[jar::M2S1FSRBModalF::io::<()>()] *= fluid_damping_gain;
        }
        /*print!(
            "#{:2}   {:+6.4}  {:+6.4}",
            k,
            Option::<Vec<f64>>::from(&y.clone().unwrap()[jar::M2S1FSCPModalF::io::<()>()]).unwrap()
                [0],
            Option::<Vec<f64>>::from(&y.clone().unwrap()[jar::M2S1FSRBModalF::io::<()>()]).unwrap()
                [0],
        );*/

        fem = dms.in_step_out(y)?;
        if let Some(ref x) = fem {
            modal_fs.push(x[jar::M2S1FSModalD::io::<()>()].clone());
            /*if k % decimation == 0 {
                modal_fs.push(x[jar::M2S1FSModalD::io::<()>()].clone());
            }*/
            /*println!(
                "  {:7.4}  {:7.4}",
                1e6 * Option::<Vec<f64>>::from(&x[jar::M2S1FSModalD::io::<()>()]).unwrap()[0],
                1e6 * Option::<Vec<f64>>::from(&x[jar::M2S1FSRBModalD::io::<()>()]).unwrap()[0],
            );*/
        };
    }

    let elapsed = now.elapsed().as_millis();
    println!(
        "Elapsed time: {}ms [{:.3}ms/step]",
        elapsed,
        elapsed as f64 / n_step as f64
    );

    let time_series: Vec<_> = modal_fs
        .iter()
        .map(|y| Option::<Vec<f64>>::from(y).unwrap())
        .collect();
    let piston: Vec<_> = time_series.iter().map(|x| x[0] * 1e6).collect();
    let piston_max = piston.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let piston_min = piston.iter().cloned().fold(f64::INFINITY, f64::min);
    println!("Piston min/max: [{},{}]", piston_min, piston_max);

    let root_area = BitMapBackend::new("modal_facesheet.png", (1200, 800)).into_drawing_area();
    root_area.fill(&WHITE)?;

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

    let color = colors.next().unwrap().as_tuple();
    let rgb = RGBColor(color.0, color.1, color.2);
    ctx.draw_series(LineSeries::new(
        piston
            .iter()
            .enumerate()
            .map(|(k, &x)| (k as f64 / sampling_rate, x)),
        &rgb,
    ))?;

    Ok(())
}
