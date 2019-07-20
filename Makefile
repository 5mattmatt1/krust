#
# MIT License
#
# Copyright (c) 2018-2019 
# 2018-2019 Matthew Henderson <mattw2018@hotmail.com>
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.
#

TARGET = aarch64-unknown-none
# TARGET = arm7-unknown-none

XRUSTC_FLAGS = --target=$(TARGET)
# --release
XRUSTC_CMD = cargo xrustc 

# KERNEL = krust8
KERNEL = krust
ARM_VER = 8
KERNEL_IMG = ${KERNEL}${ARM_VER}.img

SOURCES = $(wildcard **/*.rs) $(wildcard **/*.S) link.ld

OBJCOPY = cargo objcopy --
OBJCOPY_PARAMS = --strip-all -O binary

DOC_PARAMS = --document-private-items --document-private-items

UTILS_CONTAINER = andrerichter/raspi3-utils
DOCKER_CMD = docker run -it --rm -v $(shell pwd):/work -w /work
QEMU_CMD = qemu-system-aarch64 -M raspi3 -kernel $(KERNEL_IMG)

DEBUG = target/$(TARGET)/debug/$(KERNEL)
RELEASE = target/$(TARGET)/release/$(KERNEL)

# TODO: Read up on Make-fu and get it so that
# it creates a set of targets based off of a list of tests in the src/bin directory
TEST_DMA = target/$(TARGET)/debug/test_dma
TEST_DMA_IMG = test_dma.img
TEST_SDHCI = target/$(TARGET)/debug/test_sdhci
TEST_SDHCI_IMG = test_sdhci.img
TEST_MAILBOX = target/$(TARGET)/debug/test_mailbox
TEST_MAILBOX_IMG = test_mailbox.img

.PHONY: all qemu clippy clean objdump nm

all: clean $(KERNEL_IMG)

tests: $(TEST_DMA_IMG) $(TEST_SDHCI_IMG) $(TEST_MAILBOX_IMG)

$(DEBUG): $(SOURCES)
	cargo xbuild --target=$(TARGET)

$(KERNEL_IMG): $(DEBUG)
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

$(TEST_DMA_IMG): $(TEST_DMA) $(DEBUG)
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

$(TEST_SDHCI_IMG): $(TEST_SDHCI) $(DEBUG)
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

$(TEST_MAILBOX_IMG): $(TEST_MAILBOX) $(DEBUG)
	$(OBJCOPY) $(OBJCOPY_PARAMS) $< $@

qemu: all
	$(DOCKER_CMD) $(UTILS_CONTAINER) $(QEMU_CMD) -serial null -serial stdio

clippy:
	cargo xclippy --target=$(TARGET)

check:
	cargo xcheck --target=$(TARGET)

doc:
	cargo doc

clean:
	cargo clean

objdump:
	cargo objdump --target $(TARGET) -- -disassemble -print-imm-hex $(KERNEL)

nm:
	cargo nm --target $(TARGET) -- $(KERNEL) | sort
