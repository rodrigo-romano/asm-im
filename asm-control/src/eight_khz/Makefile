.PHONY: new

SEGMENT = segment$(ID)

new:
	echo S$(ID)
	mkdir -p $(SEGMENT)
	sed 's/ASM_modal_controller_S1/ASM_modal_controller_S$(ID)/g' segment1/ASM_modal_controller_S1.h > $(SEGMENT)/ASM_modal_controller_S$(ID).h
	sed 's/ASM_modal_controller_S1/ASM_modal_controller_S$(ID)/g' segment1/ASM_modal_controller_S1_private.h > $(SEGMENT)/ASM_modal_controller_S$(ID)_private.h
	sed 's/ASM_modal_controller_S1/ASM_modal_controller_S$(ID)/g' segment1/ASM_modal_controller_S1_types.h > $(SEGMENT)/ASM_modal_controller_S$(ID)_types.h
	sed 's/ASM_modal_controller_S1/ASM_modal_controller_S$(ID)/g' segment1/rtwtypes.h > $(SEGMENT)/rtwtypes.h
	sed 's/ASM_modal_controller_S1/ASM_modal_controller_S$(ID)/g' segment1/ASM_modal_controller_S1.c > $(SEGMENT)/ASM_modal_controller_S$(ID).c
	sed 's/ASM_modal_controller_S1/ASM_modal_controller_S$(ID)/g' segment1/ASM_modal_controller_S1_data.c > $(SEGMENT)/ASM_modal_controller_S$(ID)_data.c
	sed 's/S1/S$(ID)/g' segment1/mod.rs > $(SEGMENT)/mod.rs
