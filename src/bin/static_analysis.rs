use complot as plt;
use fem::FEM;
use indicatif::ProgressBar;
use nalgebra as na;
use plotters::prelude::*;
use std::{env, fs::File, path::Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Finite element model
    let fem_data_path_var = env::var("FEM_DATA_PATH")
        .map_err(|e| format!("FEM_DATA_PATH env var not set! Caused by: {}", e))?;

    let fem_data_path = Path::new(&fem_data_path_var);
    let fem_state_space_model = if let Ok(var) = env::var("FEM_MODEL") {
        var
    } else {
        "modal_state_space_model_2ndOrder.73.pkl".to_string()
    };
    let actuators_locations: Vec<Vec<Vec<f64>>> = {
        let mut fem = FEM::from_pickle(&fem_data_path.join(fem_state_space_model))?;
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
    // FEM static gain
    let file = File::open("/media/rconan/FEM/20210614_2105_ASM_topendOnly/gainMatrix.mat")?;
    let mat_file = matfile::MatFile::parse(file)?;
    let gain =
        {
            let gain = mat_file.find_by_name("gainMatrix").unwrap();
            println!("gain: {:?}", gain.size());
            match gain.data() {
                matfile::NumericData::Double { real, imag: _ } => Some(
                    na::DMatrix::from_column_slice(gain.size()[0], gain.size()[1], &real),
                ),
                _ => None,
            }
            .unwrap()
        };
    let zernikes: Vec<_> = (1..=7)
        .filter_map(|i| mat_file.find_by_name(&format!("P{}", i)))
        .enumerate()
        .map(|(k, z)| {
            println!("Z{}: {:?}", k, z.size());
            match z.data() {
                matfile::NumericData::Double { real, imag: _ } => Some(
                    na::DMatrix::from_column_slice(z.size()[0], z.size()[1], &real),
                ),
                _ => None,
            }
            .unwrap()
        })
        .collect();

    let i: usize = 6 + 84 + 42 * 4;
    let o: usize = 84 + 42 * 4 + 675 * 7;
    let n: usize = 675;

    let segment_gains: Vec<_> = (0..7)
        .map(|k| gain.slice((o + k * n, i + 2 * k * n), (n, n)))
        .collect();

    let coordinates: Vec<Vec<(f64, f64)>> = actuators_locations
        .iter()
        .map(|actuators_location| {
            actuators_location
                .iter()
                .map(|point| (point[0], point[1]))
                .collect()
        })
        .collect();

    let coord = &coordinates[0];
    let (x_segment, y_segment): (Vec<_>, Vec<_>) = coord.iter().copied().unzip();

    let pb = ProgressBar::new(n as u64);
    for (k, col) in segment_gains[0].column_iter().enumerate() {
        pb.inc(1);
        let filename = format!(
            "data/influence_functions/zonal/influence_function_{:03}.png",
            k
        );
        let root_area = BitMapBackend::new(&filename, (800, 800)).into_drawing_area();
        let lim = 0.55f64;
        let mut ctx = ChartBuilder::on(&root_area)
            .margin(20)
            //.set_label_area_size(LabelAreaPosition::Left, 20)
            //.set_label_area_size(LabelAreaPosition::Bottom, 20)
            .caption(
                format!("Influence Function #{}", k + 1),
                ("sans-serif", 40, &WHITE),
            )
            .build_cartesian_2d(-lim..lim, -lim..lim)?;
        ctx.configure_mesh().draw()?;

        plt::trimap(&x_segment, &y_segment, &col.as_slice(), &mut ctx);
        ctx.draw_series(
            coord
                .iter()
                .map(|&point| Circle::new(point, 2, BLACK.mix(0.25))),
        )
        .unwrap();
    }
    pb.finish();

    let pb = ProgressBar::new(66);
    let modal_gain = segment_gains[0] * &zernikes[0];
    for (k, col) in modal_gain.column_iter().enumerate() {
        pb.inc(1);
        let filename = format!(
            "data/influence_functions/modal/influence_function_{:03}.png",
            k
        );
        let root_area = BitMapBackend::new(&filename, (800, 800)).into_drawing_area();
        let lim = 0.55f64;
        let mut ctx = ChartBuilder::on(&root_area)
            .margin(20)
            //.set_label_area_size(LabelAreaPosition::Left, 20)
            //.set_label_area_size(LabelAreaPosition::Bottom, 20)
            .caption(format!("Zernike #{}", k + 1), ("sans-serif", 40, &WHITE))
            .build_cartesian_2d(-lim..lim, -lim..lim)?;
        ctx.configure_mesh().draw()?;

        plt::trimap(&x_segment, &y_segment, &col.as_slice(), &mut ctx);
        ctx.draw_series(
            coord
                .iter()
                .map(|&point| Circle::new(point, 2, BLACK.mix(0.25))),
        )
        .unwrap();
    }
    pb.finish();

    Ok(())
}
