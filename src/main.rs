use asm_control::ASMS;
use colorous;
use complot as plt;
use dosio::{ios, Dos, IOTags, IO};
use fem::{dos, FEM};
use indicatif::ProgressBar;
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
    /// GIF animation flag
    #[structopt(long)]
    gif: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    // ASM controller
    let logs_data_tags = ios!(
        M2segment1axiald,
        M2segment2axiald,
        M2segment3axiald,
        M2segment4axiald,
        M2segment5axiald,
        M2segment6axiald,
        M2segment7axiald
    );
    let mut asms = ASMS::new().modal_forces_gain(opt.modal_forces_gain);
    // ASM command input (segment #1)
    let mut asm_cmd = vec![0f64; 66];
    asm_cmd[0] = 1e-6 / 0.03849;

    // Finite element model
    let actuators_locations: Vec<Vec<Vec<f64>>> = {
        let mut fem = FEM::from_pickle(
        "/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder.73.pkl",
    )?;
        fem.keep_inputs(&[usize::MAX]);
        fem.keep_outputs(&[1, 2, 3, 4, 5, 6, 7]);
        println!("FEM: {}", fem);
        fem.outputs
            .iter()
            .filter_map(|output| {
                output
                    .as_ref()
                    .map(|values| values.get_by(|x| x.properties.location.clone()))
            })
            .collect()
    };
    println!(
        "Actuators: {}/{}/{}",
        actuators_locations.len(),
        actuators_locations[0].len(),
        actuators_locations[0][0].len()
    );
    let snd_ord = {
        let snd_ord = dos::SecondOrder::from_pickle("/media/rconan/FEM/20210614_2105_ASM_topendOnly/modal_state_space_model_2ndOrder_1500Hz_noRes_postproc_v1.pkl")?;
        println!("{}", &snd_ord);
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
    let decimation = 80_usize;
    let mut modal_fs: Vec<IO<Vec<f64>>> =
        Vec::with_capacity(logs_data_tags.len() * n_step / decimation);
    let pb = ProgressBar::new(n_step as u64);
    let now = Instant::now();
    for k in 0..n_step {
        pb.inc(1);
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
            if k % decimation == 0 {
                logs_data_tags.iter().for_each(|t| {
                    modal_fs.push(x[t].clone());
                });
            }
            #[cfg(debug)]
            println!(
                "  {:7.4}  {:7.4}",
                1e6 * Option::<Vec<f64>>::from(&x[ios!(M2S1FSModalD)]).unwrap()[0],
                1e6 * Option::<Vec<f64>>::from(&x[ios!(M2S1FSRBModalD)]).unwrap()[0],
            );
        };
    }
    pb.finish();
    let elapsed = now.elapsed().as_millis();
    println!(
        "Elapsed time: {}ms [{:.3}ms/step]",
        elapsed,
        elapsed as f64 / n_step as f64
    );

    // Post-processing:
    println!("modal_fs: {}", modal_fs.len());
    let segment_figures: Vec<Vec<f64>> = modal_fs
        .chunks(7)
        .last()
        .unwrap()
        .iter()
        .map(|y| Option::<Vec<f64>>::from(y).unwrap())
        .collect();
    println!(
        "segment_figures: {}/{}",
        segment_figures.len(),
        segment_figures[0].len()
    );
    //  - segment piston
    let segment_piston: Vec<_> = segment_figures
        .iter()
        .map(|fig| 1e6 * fig.iter().sum::<f64>() / fig.len() as f64)
        .collect();
    println!("Segment piston: {:7.3?}micron", segment_piston);
    //  - pupil piston
    let pupil_piston = segment_piston.iter().sum::<f64>() / 7f64;
    println!("Pupil piston: {:7.3}micron", pupil_piston);
    //  - segment surface std
    let segment_surface_std: Vec<_> = segment_figures
        .iter()
        .zip(segment_piston.iter())
        .map(|(fig, pist)| {
            fig.iter().map(|x| (1e6 * x - pist).powi(2)).sum::<f64>() / fig.len() as f64
        })
        .map(|x| 1e3 * x.sqrt())
        .collect();
    println!("Segment surface STD: {:4.0?}nm", segment_surface_std);
    //  - pupil surface std
    let surface_var = segment_figures
        .iter()
        .map(|fig| {
            fig.iter()
                .map(|x| (1e6 * x - pupil_piston).powi(2))
                .sum::<f64>()
        })
        .sum::<f64>()
        / (7f64 * 675f64);
    println!("Pupil surface STD: {:4.0?}nm", 1e3 * surface_var.sqrt());

    if opt.gif {
        let outer_radial_center = 1.1f64;
        let coordinates: Vec<Vec<(f64, f64)>> = actuators_locations
            .iter()
            .enumerate()
            .map(|(k, actuators_location)| {
                let (x0, y0) = if k < 6 {
                    {
                        let o = 2f64 * std::f64::consts::PI * (k as f64 / 6. + 0.25);
                        (outer_radial_center * o.cos(), outer_radial_center * o.sin())
                    }
                } else {
                    (0f64, 0f64)
                };
                actuators_location
                    .iter()
                    .map(move |point| (x0 + point[0], y0 + point[1]))
                    .collect()
            })
            .collect();
        //    let root_area = BitMapBackend::new("nodal_facesheet.png", (800, 880)).into_drawing_area();
        let root_area = BitMapBackend::gif("nodal_facesheet.gif", (800, 880), 1_000)
            .unwrap()
            .into_drawing_area();

        for (i, actuators_disp) in modal_fs.chunks(7).enumerate() {
            let segment_figures: Vec<Vec<f64>> = actuators_disp
                .iter()
                .map(|y| Option::<Vec<f64>>::from(y).unwrap())
                .collect();

            root_area.fill(&WHITE)?;
            let (plot, colorbar) = root_area.split_vertically(800);
            let lim = 1.75f64;
            let mut ctx = ChartBuilder::on(&plot)
                .margin(20)
                .set_label_area_size(LabelAreaPosition::Left, 20)
                .set_label_area_size(LabelAreaPosition::Bottom, 20)
                .caption(
                    format!("T = {:7.3}s", i as f64 * decimation as f64 / sampling_rate),
                    ("sans-serif", 16, &BLACK),
                )
                //.caption("Line Plot Demo", ("sans-serif", 40))
                //.build_cartesian_2d(0f64..n_step as f64 / sampling_rate, -1e-3f64..10e-3f64)
                .build_cartesian_2d(-lim..lim, -lim..lim)?;
            ctx.configure_mesh().draw()?;
            let mut max_figs = vec![];
            let mut min_figs = vec![];
            for (coord, fig) in coordinates.iter().zip(segment_figures.iter()) {
                let (x, y): (Vec<_>, Vec<_>) = coord.iter().copied().unzip();
                max_figs.push(fig.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
                min_figs.push(fig.iter().cloned().fold(f64::INFINITY, f64::min));
                plt::trimap(&x, &y, fig, &mut ctx);
                ctx.draw_series(
                    coord
                        .iter()
                        .map(|&point| Circle::new(point, 2, BLACK.mix(0.25))),
                )
                .unwrap();
            }

            // COLORBAR
            let cells_min = 1e6 * min_figs.into_iter().fold(f64::INFINITY, f64::min);
            let cells_max = 1e6 * max_figs.into_iter().fold(f64::NEG_INFINITY, f64::max);
            colorbar.fill(&BLACK)?;
            let mut colorbar_chart = ChartBuilder::on(&colorbar)
                //    .margin_left(20)
                //    .margin_right(20)
                .set_label_area_size(LabelAreaPosition::Bottom, 40)
                .build_cartesian_2d(cells_min..cells_max, 0f64..1f64)?;
            let mut mesh = colorbar_chart.configure_mesh();
            mesh.axis_style(WHITE)
                .set_tick_mark_size(LabelAreaPosition::Bottom, 5)
                .x_label_style(("sans-serif", 14, &WHITE))
                .x_desc("WFE [micron]")
                .draw()?;
            let dx = (cells_max - cells_min) / (800 - 1) as f64;
            let cmap = colorous::CIVIDIS;
            colorbar_chart.draw_series((0..800).map(|k| {
                let x = cells_min + k as f64 * dx;
                let c = cmap.eval_rational(k, 800).as_tuple();
                Rectangle::new([(x, 0.), (x + dx, 1.)], RGBColor(c.0, c.1, c.2).filled())
            }))?;
            root_area.present()?;
        }
    }
    Ok(())
}
