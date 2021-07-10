use asm_control::ASMS;
use dosio::{ios, Dos};
fn main() {
    let mut asm_cmd = vec![0f64; 66];
    asm_cmd[0] = 1e-6;

    let mut asms: ASMS = vec![1, 2].into();

    for k in 0..5 {
        let u = ios!(
            M2S1FSRBModalD(vec![0f64; 66]),
            M2S1Cmd(asm_cmd.clone()),
            M2S2FSRBModalD(vec![0f64; 66]),
            M2S2Cmd(vec![0f64; 66])
        );
        let y = asms.in_step_out(Some(u));
        if let Ok(y) = y {
            println!(
                "#{:2}   {:+6.4}  {:+6.4}   {:+6.4}  {:+6.4}",
                k,
                Option::<Vec<f64>>::from(&y.clone().unwrap()[ios!(M2S1FSCPModalF)]).unwrap()[0],
                Option::<Vec<f64>>::from(&y.clone().unwrap()[ios!(M2S1FSRBModalF)]).unwrap()[0],
                Option::<Vec<f64>>::from(&y.clone().unwrap()[ios!(M2S2FSCPModalF)]).unwrap()[0],
                Option::<Vec<f64>>::from(&y.clone().unwrap()[ios!(M2S2FSRBModalF)]).unwrap()[0],
            );
        }
    }
}
