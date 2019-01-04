OBJCOPY=arm-none-eabi-objcopy
OBJDUMP=arm-none-eabi-objdump

TARGET=arm-unknown-linux-gnueabihf

SOURCES := $(shell find src -name '*.rs')
# Taken from https://github.com/osnr/rpi-kernel
# Files
NAME=kernel

.PHONY: build clean listing $(OUT_FILE)

all: clean krust.img krust.list

krust.img: krust.elf
	$(OBJCOPY) krust.elf -O binary krust.img

krust.list: krust.img
	$(OBJDUMP) -d krust.elf > krust.list

# src/interrupts_asm.s
krust.elf: src/start.o src/main.o
	arm-none-eabi-gcc -O0 -g -Wl,-gc-sections -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostdlib $^ -o $@

%.o: %.rs $(SOURCES)
	rustc --target arm-unknown-linux-gnueabihf -g --crate-type="staticlib" $< -o $@

%.o: %.s
	arm-none-eabi-as $< -o $@

# install: clean kernel.img
#	rpi-install.py kernel.img

# install-screen: install
#	sleep 5
#	screen /dev/tty.SLAB_USBtoUART 115200

clean:
	rm -f krust.img
	rm -f krust.elf
	rm -f src/*.o
