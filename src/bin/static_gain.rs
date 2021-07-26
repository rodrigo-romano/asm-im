use complot::{triplot::heatmap, Config};
use fem::FEM;
use geotrans::{Quaternion, Vector};
use nalgebra as na;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fem = FEM::from_pickle(
        "/media/rconan/FEM/20210614_2105_ASM_topendOnly/static_reduction_model.73.pkl",
    )?;
    println!("Static FEM:\n{}", fem);
    let n_io = (fem.n_inputs(), fem.n_outputs());
    fem.keep_inputs_by(&[19], |x| x.descriptions.starts_with("M2-S7"));
    fem.keep_outputs(&[7, 19]);
    fem.filter_outputs_by(&[19], |x| x.descriptions.starts_with("M2-S7"));
    let gain = fem.reduced_static_gain(n_io).unwrap();
    //println!("Gain: {:#?}", gain);
    println!("Static FEM:\n{}", fem);
    let mut u = vec![0f64; 6];
    u[3] = 10f64;
    println!("u: {:#?}", u);
    let u_vec = na::DVector::from_column_slice(&u);
    let y = (1e6 * gain * u_vec).as_slice().to_vec();
    println!("<y[..675]>: {:#?}", &y[..675].iter().sum::<f64>() / 675_f64);
    println!("y[675..]: {:#?}", &y[675..]);
    /*println!(as
                "<y[..675] - rbm>: {:#?}",
                1e3 * &y[..675].iter().map(|x| x - y[677]).sum::<f64>() / 675_f64
    );*/

    {
        let nodes_z_minmax = fem.outputs[7]
            .as_ref()
            .unwrap()
            .get_by(|x| x.properties.location.clone())
            .iter()
            .map(|node| node[2])
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), x| {
                (f64::min(min, x), f64::max(max, x))
            });
        println!("Actuators locations z-range: {:.6?}m", nodes_z_minmax);
    }

    {
        let (x_nodes, y_nodes): (Vec<f64>, Vec<f64>) = fem.outputs[7]
            .as_ref()
            .unwrap()
            .get_by(|x| x.properties.location.clone())
            .iter()
            .map(|point| (point[0], point[1]))
            .unzip();
        let cfg = Config::new().filename("axial_displacement_1.png");
        heatmap(&x_nodes, &y_nodes, &y[..675], -0.55f64..0.55f64, Some(cfg))?;
    }

    {
        let rbm = &y[675..].to_vec();
        let q = Quaternion::unit(0.1, Vector::i());
        //* Quaternion::unit(rbm[4], Vector::j())
        //* Quaternion::unit(rbm[5], Vector::k());
        let t: Vector = rbm[..3].into();
        let axial_deformations: Vec<_> = fem.outputs[7]
            .as_ref()
            .unwrap()
            .get_by(|x| x.properties.location.clone())
            .iter()
            .zip(&y[..675])
            .map(|(node, delta_z)| {
                let n: Quaternion = Vector::from(node.as_slice()).into();
                let t: Vector = Vector::from(rbm[..3].to_vec());
                let m = &q * n * q.complex_conjugate(); // + t.clone().into();
                                                        //let mut mm = Vector::from(m.vector_as_slice());
                                                        //mm[2] = *delta_z;
                                                        //let p = q.complex_conjugate()*(Quaternion::from(mm) - t.into())*&q;
                Vector::from(m.vector_as_slice())
                //mm[2]
            })
            .collect();

        {
            /*let (x_nodes, y_nodes): (Vec<f64>, Vec<f64>) = fem.outputs[7]
                .as_ref()
                .unwrap()
                .get_by(|x| x.properties.location.clone())
                .iter()
                .map(|point| (point[0], point[1]))
            .unzip();*/
            let (xy, z): (Vec<_>, Vec<_>) = axial_deformations
                .iter()
                .map(|node| ((node[0], node[1]), 1e6 * node[2]))
                .unzip();
            let (x, y): (Vec<_>, Vec<_>) = xy.into_iter().unzip();
            let nodes_z_minmax = z
                .iter()
                .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), &x| {
                    (f64::min(min, x), f64::max(max, x))
                });
            println!("Actuators locations z-range: {:.6?}m", nodes_z_minmax);
            let cfg = Config::new().filename("axial_displacement_2.png");
            heatmap(&x, &y, &z, -0.55f64..0.55f64, Some(cfg))?;
        }
    }

    Ok(())
}
