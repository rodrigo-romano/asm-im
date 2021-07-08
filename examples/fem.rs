use asm_control;
use colorous;
use dosio::{io::jar, Dos};
use fem::dos;
use plotters::prelude::*;
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
            vec![jar::M2S1FSCPModalF::io(), jar::M2S1FSRBModalF::io()],
            vec![jar::M2S1FSRBModalD::io(), jar::M2S1FSModalD::io()],
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

    let mut asm_ctrl = asm_control::Controller::new(asm_control::Segment::One);

    let mut asm_cmd = vec![0f64; 66];
    //asm_cmd[0] = 1e-6;
    asm_cmd[0] = 0.389204179097122;
    let mut asm = Some(vec![
        jar::M2S1FSCPModalF::io_with(asm_cmd),
        jar::M2S1FSRBModalF::io_with(vec![0f64; 66]),
    ]);
    /*
       for i in 0..10 {
           let fem = dms.in_step_out(asm.clone())?.unwrap();
           let y0: Option<Vec<f64>> = fem[0].clone().into();
           let y1: Option<Vec<f64>> = fem[1].clone().into();
           let sum = y0.unwrap().iter().sum::<f64>() + y1.unwrap().iter().sum::<f64>();
           println!("#{:2}: {}", i, sum);
       }
    */

    let now = Instant::now();
    let n_step = 20_u128;
    let mut y = vec![];
    for k in 0..n_step {
        //println!("{:#?}", asm);
        let fem = dms.in_step_out(asm.clone())?.map(|mut x| {
            let y0 = Option::<Vec<f64>>::from(&x[jar::M2S1FSModalD::io::<()>()]).unwrap();
            let y1 = Option::<Vec<f64>>::from(&x[jar::M2S1FSRBModalD::io::<()>()]).unwrap();
            println!(
                "#{:2}: {:8.4}, {:8.4} {:8.4}, {:8.4}",
                k,
                1e6 * y0[0],
                1e6 * y1[0],
                1e6 * y0.iter().sum::<f64>(),
                1e6 * y1.iter().sum::<f64>(),
            );
            y.push(x[jar::M2S1FSModalD::io::<()>()].clone());
            x
        });
    }
    let elapsed = now.elapsed().as_millis();
    println!(
        "Elapsed time: {}ms [{:.3}ms/step]",
        elapsed,
        elapsed as f64 / n_step as f64
    );

    //let mut y_last = y.last().unwrap().clone();
    //&mut y_last * 1e6f64;
    //println!("last y: {:#?}", y_last);

    let time_series: Vec<_> = y
        .iter()
        .map(|y| Option::<Vec<f64>>::from(y).unwrap())
        .collect();
    let piston: Vec<_> = time_series.iter().map(|x| x[0] * 1e6).collect();
    let piston_max = piston.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let piston_min = piston.iter().cloned().fold(f64::INFINITY, f64::min);
    println!("Piston min/max: [{},{}]", piston_min, piston_max);

    let root_area = SVGBackend::new("modal_facesheet.svg", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .margin(20)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        //.caption("Line Plot Demo", ("sans-serif", 40))
        //.build_cartesian_2d(0f64..n_step as f64 / sampling_rate, -1e-3f64..10e-3f64)
        .build_cartesian_2d(0f64..n_step as f64 / sampling_rate, -1f64..3f64)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let mut colors = colorous::TABLEAU10.iter().cycle();

    let color = colors.next().unwrap().as_tuple();
    let rgb = RGBColor(color.0, color.1, color.2);
    ctx.draw_series(LineSeries::new(
        piston
            .iter()
            .enumerate()
            .map(|(k, &x)| (k as f64 / sampling_rate, x)),
        &rgb,
    ))
    .unwrap();

    /*
        let fem_data_path = path::Path::new("/")
            .join("media")
            .join("rconan")
            .join("FEM")
            .join("20210614_2105_ASM_topendOnly");
        let sampling_rate = 8e3;
        let mut fem = {
            let fem = FEM::from_pickle(fem_data_path.join("modal_state_space_model_2ndOrder.73.pkl"))?;
            println!("FEM\n{}", fem);

            DiscreteStateSpace::from(fem)
        }
        .sampling(sampling_rate)
        .inputs(vec![
            jar::MCM2S1VCDeltaF::io(),
            jar::MCM2S1FluidDampingF::io(),
        ])
        .outputs(vec![jar::M2Segment1AxialD::io(), jar::MCM2S1VCDeltaD::io()])
        .build()?;
    */
    Ok(())
}
