use complot as plt;
use complot::Plot;
use fem::FEM;
use plotters::prelude::*;
use plt::triplot::TriPlot;
use triangle_rs as tri;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fem = FEM::from_pickle(
        "/media/rconan/FEM/20210614_2105_ASM_topendOnly/static_reduction_model.73.pkl",
    )?;
    println!("Static FEM:\n{}", fem);
    fem.keep_inputs_by(&[19], |x| x.descriptions.starts_with("M2-S1"));
    fem.keep_outputs(&[1, 19]);
    fem.filter_outputs_by(&[19], |x| x.descriptions.starts_with("M2-S1"));
    println!("Static FEM:\n{}", fem);

    let nodes: Vec<_> = fem.outputs[1]
        .as_ref()
        .unwrap()
        .get_by(|x| x.properties.location.clone())
        .iter()
        .flat_map(|point| point[..2].to_vec())
        .collect();

    println!(
        "Nodes max diameter: {:.1}mm",
        2e3 * nodes
            .chunks(2)
            .map(|xy| xy[0].hypot(xy[1]))
            .fold(f64::NEG_INFINITY, f64::max),
    );

    let rim_diameter = 1.0503;
    let delta_rim = 5e-2;
    let n_rim = (std::f64::consts::PI * rim_diameter / delta_rim).round();
    let outer_rim: Vec<_> = (0..n_rim as usize)
        .flat_map(|i| {
            let o = 2. * std::f64::consts::PI * i as f64 / n_rim;
            let (s, c) = o.sin_cos();
            let radius = 0.5 * rim_diameter;
            vec![radius * c, radius * s]
        })
        .collect();

    let mut builder = tri::Builder::new().set_tri_points(nodes.clone());
    builder.add_polygon(&outer_rim).set_switches("QDqa0.0003");
    let del = builder.build();
    println!("{}", del);
    del.dump("m2-s1_delaunay.pkl")?;
    let fig = plt::canvas("m2-s1_delaunay.svg");
    let lim = 0.55_f64;
    let mut ax = plt::chart([-lim, lim, -lim, lim], &fig);
    del.mesh(&del.x(), &del.y(), [0; 3], &mut ax);
    ax.draw_series(
        nodes
            .chunks(2)
            .map(|point| Circle::new((point[0], point[1]), 3, RED.filled())),
    )?;
    //Plot::from(nodes.as_slice());

    Ok(())
}
