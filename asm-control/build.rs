use cc;

fn main() {
    cc::Build::new()
        .file("src/eight_khz/segment1/ASM_modal_controller_S1.c")
        .file("src/eight_khz/segment1/ASM_modal_controller_S1_data.c")
        .compile("asm_controller_8k_1");
    cc::Build::new()
        .file("src/eight_khz/segment2/ASM_modal_controller_S2.c")
        .file("src/eight_khz/segment2/ASM_modal_controller_S2_data.c")
        .compile("asm_controller_8k_2");
    cc::Build::new()
        .file("src/eight_khz/segment3/ASM_modal_controller_S3.c")
        .file("src/eight_khz/segment3/ASM_modal_controller_S3_data.c")
        .compile("asm_controller_8k_3");
    cc::Build::new()
        .file("src/eight_khz/segment4/ASM_modal_controller_S4.c")
        .file("src/eight_khz/segment4/ASM_modal_controller_S4_data.c")
        .compile("asm_controller_8k_4");
    cc::Build::new()
        .file("src/eight_khz/segment5/ASM_modal_controller_S5.c")
        .file("src/eight_khz/segment5/ASM_modal_controller_S5_data.c")
        .compile("asm_controller_8k_5");
    cc::Build::new()
        .file("src/eight_khz/segment6/ASM_modal_controller_S6.c")
        .file("src/eight_khz/segment6/ASM_modal_controller_S6_data.c")
        .compile("asm_controller_8k_6");
    cc::Build::new()
        .file("src/eight_khz/segment7/ASM_modal_controller_S7.c")
        .file("src/eight_khz/segment7/ASM_modal_controller_S7_data.c")
        .compile("asm_controller_8k_7");
    cc::Build::new()
        .file("src/fourty_khz/ASM_modal_controller_S1.c")
        .file("src/fourty_khz/ASM_modal_controller_S1_data.c")
        .compile("asm_controller_40k");
}
