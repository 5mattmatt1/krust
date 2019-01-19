OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-unknown-linux-gnueabihf

SOURCES := $(shell find src -name '*.rs')
# Taken from https://github.com/osnr/rpi-kernel
# Files
NAME=krust

.PHONY: build clean listing $(OUT_FILE)

all: clean krust.img krust.list

uart:
	sudo tail -f /dev/ttyUSB0

install: krust.img
	cp krust.img '/media/grainne-root/PI_BOOT/'

krust.img: krust.elf
	$(OBJCOPY) krust.elf -O binary krust.img

krust.list: krust.img
	$(OBJDUMP) -d krust.elf > krust.list

# src/interrupts_asm.s
krust.elf: src/start.o src/main.o
	arm-none-eabi-gcc -T krust.ld -O0 -g -Wl,-gc-sections -O2 -nostartfiles -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -mtune=cortex-a7 -nostdlib $^ -o $@

# opt-level=s Makes some statuses fail...
%.o: %.rs $(SOURCES)
	rustc --target arm-unknown-linux-gnueabihf -g --crate-type="staticlib" -C lto=fat -C opt-level=0 $< -o $@

%.o: %.s
	arm-none-eabi-as -mcpu=cortex-a7 $< -o $@

# install: clean kernel.img
#	rpi-install.py kernel.img

# install-screen: install
#	sleep 5
#	screen /dev/tty.SLAB_USBtoUART 115200

clean:
	rm -f krust.img
	rm -f krust.elf
	rm -f src/*.o
