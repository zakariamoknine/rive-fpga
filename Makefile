RISCV_CROSS_COMPILE ?= riscv64-unknown-linux-gnu-
CC                   = $(RISCV_CROSS_COMPILE)gcc
LD                   = $(RISCV_CROSS_COMPILE)ld
OBJCOPY              = $(RISCV_CROSS_COMPILE)objcopy

DTC ?= dtc

BUILD_DIR := $(PWD)/build

SBI_BUILD_DIR      := $(BUILD_DIR)/opensbi
KERNEL_BUILD_DIR   := $(BUILD_DIR)/rive
FIRMWARE_BUILD_DIR := $(BUILD_DIR)/firmware
DTB_BUILD_DIR      := $(BUILD_DIR)/dtb

all: sbi

sbi: kernel
	@mkdir -p $(dir $(SBI_BUILD_DIR))
	$(MAKE) -C submodules/opensbi \
		PLATFORM=generic \
		CROSS_COMPILE=$(RISCV_CROSS_COMPILE) \
		FW_TEXT_START=0x80000000 \
		FW_JUMP_ADDR=0x82000000 \
		O=$(SBI_BUILD_DIR)

kernel: firmware
	@mkdir -p $(dir $(KERNEL_BUILD_DIR))
	$(MAKE) -C submodules/rive ARCH=riscv64 O=$(KERNEL_BUILD_DIR)

firmware:
	@mkdir -p $(dir $(FIRMWARE_BUILD_DIR))
	$(MAKE) -C firmware O=$(FIRMWARE_BUILD_DIR)

serial_boot:
	./scripts/serial_boot.py \
		$(KERNEL_BUILD_DIR)/kernel.bin \
	       	-p /dev/ttyUSB1 -b 115200
	./scripts/serial_boot.py \
		$(SBI_BUILD_DIR)/platform/generic/firmware/fw_jump.bin \
	       	-p /dev/ttyUSB1 -b 115200

bram:
	./scripts/update_bram.sh $(FIRMWARE_BUILD_DIR)/firmware.mem

cpu:
	./scripts/generate_cpu.sh
	mv submodules/VexiiRiscv/VexiiRiscv.v ip/cpu/VexiiRiscv.v

clean:
	rm -rf $(BUILD_DIR)

.PHONY: all sbi kernel firmware serial_boot bram cpu clean
