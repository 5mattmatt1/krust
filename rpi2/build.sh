# TODO: Turn this into a proper makefile...
# rustc --target armv7-unknown-linux-gnueabihf --O --emit=obj ./src/main.rs
# arm-linux-gnueabihf-gcc -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles main.o -o krust.elf
# arm-linux-gnueabihf-objcopy krust.elf -O binary krust.img

RUST_TARGET_PATH=$(pwd) xargo rustc --target arm-none-eabihf -- -Z pre-link-arg=-nostartfiles -Z pre-link-arg=-O0 --O
# --emit=obj
arm-none-eabi-objcopy ./target/arm-none-eabihf/debug/krust -O binary krust.img
# Look into adding a modfiied version of this into ./cargo/config
# [target.aarch64-unknown-none]
# rustflags = [
#   "-C", "link-arg=-Tlink.ld",
#  "-C", "target-feature=-fp-armv8",
#  "-C", "target-cpu=cortex-a53",
# ]

# RUST_TARGET_PATH=$(pwd) xargo build --target arm-none-eabihf
# arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -c boot.S -o boot.o
# arm-none-eabi-gcc -T kernel.ld -o krust.elf -ffreestanding -O2 -nostdlib boot.o ./target/arm-none-eabihf/debug/libkrust.rlib
# arm-none-eabi-objcopy krust.elf -O binary krust.img
