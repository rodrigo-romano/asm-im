use cc;

fn main() {
    cc::Build::new()
        .file("src/eight_khz/ASM_modal_controller_S1.c")
        .file("src/eight_khz/ASM_modal_controller_S1_data.c")
        .compile("asm_controller_8k");
    cc::Build::new()
        .file("src/fourty_khz/ASM_modal_controller_S1.c")
        .file("src/fourty_khz/ASM_modal_controller_S1_data.c")
        .compile("asm_controller_40k");
}
