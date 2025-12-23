ifdef O
	BUILD_DIR := $(shell readlink -m $(O))
else
	BUILD_DIR := $(CURDIR)/build
endif

DTC     := dtc

CARGO   := cargo
RUSTC   := rustc
OBJCOPY := rust-objcopy

SRC_DIR = $(CURDIR)

LDSCRIPT  := firmware.ld
MANIFEST  := $(SRC_DIR)/Cargo.toml

TARGET    := riscv64imac-unknown-none-elf

RUSTFLAGS := -C strip=debuginfo \
	     -C relocation-model=static \
	     -C symbol-mangling-version=v0 \
	     -C codegen-units=1 \
	     -C panic=abort \
	     -C force-frame-pointers=y \
	     -C force-unwind-tables=n \
	     -C lto=n \
	     -C opt-level=2 \
	     -C target-cpu=generic-rv64 \
	     -C code-model=medium \
	     -C no-redzone \
	     -C link-arg=-T$(LDSCRIPT)

ELF       := $(BUILD_DIR)/$(basename $(TARGET))/release/rive-firmware
FIRMWARE  := $(BUILD_DIR)/firmware.bin
MEM       := $(BUILD_DIR)/firmware.mem

all: $(FIRMWARE)

$(FIRMWARE): dtb build
	$(OBJCOPY)  --strip-debug -O binary $(ELF) $(FIRMWARE)
	echo "@00000000" > $(MEM)
	xxd -ps -c4 -g4 $(FIRMWARE) >> $(MEM)

build:
	$(CARGO) $(RUSTC) --bin rive-firmware \
		--target $(TARGET) \
		--target-dir $(BUILD_DIR) \
		--manifest-path $(MANIFEST) \
		--release \
		-Z build-std=core,alloc \
		-- $(RUSTFLAGS)

dtb:
	$(DTC) -I dts -O dtb -o rive-fpga.dtb rive-fpga.dts

.PHONY: all build
