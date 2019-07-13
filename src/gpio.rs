/*
 * MIT License
 *
 * Copyright (c) 2018-2019 Matthew Henderson <mattw2018@hotmail.com>
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
const MMIO_BASE: u32 = 0x3F000000;

const GPIO_BASE: u32 = MMIO_BASE + 0x0020_0000;
/* Function? Select */
pub const GPFSEL0: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
pub const GPFSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
pub const GPFSEL2: *mut u32 = (GPIO_BASE + 0x08) as *mut u32;
pub const GPFSEL3: *mut u32 = (GPIO_BASE + 0x0C) as *mut u32;
pub const GPFSEL4: *mut u32 = (GPIO_BASE + 0x10) as *mut u32;
pub const GPFSEL5: *mut u32 = (GPIO_BASE + 0x14) as *mut u32;
/* SET */
pub const GPSET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
pub const GPSET1: *mut u32 = (GPIO_BASE + 0x20) as *mut u32;
/* CLR */
pub const GPCLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;
/* LEVEL */
pub const GPLEV0: *mut u32 = (GPIO_BASE + 0x34) as *mut u32;
pub const GPLEV1: *mut u32 = (GPIO_BASE + 0x38) as *mut u32;
/* EDS */
pub const GPEDS0: *mut u32 = (GPIO_BASE + 0x40) as *mut u32;
pub const GPEDS1: *mut u32 = (GPIO_BASE + 0x44) as *mut u32;
/* HEN - Hardware enable? */
pub const GPHEN0: *mut u32 = (GPIO_BASE + 0x64) as *mut u32;
pub const GPHEN1: *mut u32 = (GPIO_BASE + 0x68) as *mut u32;
/* PUD */
pub const GPPUD: *mut u32 = (GPIO_BASE + 0x94) as *mut u32;
/* PUD Clock */
pub const GPPUDCLK0: *mut u32 = (GPIO_BASE + 0x98) as *mut u32;
pub const GPPUDCLK1: *mut u32 = (GPIO_BASE + 0x9C) as *mut u32;

