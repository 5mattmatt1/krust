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

use crate::sdhci::RPISDIO;
use crate::sdhci::SDIO;

const BOOTABLE: u8 = 0x80;

pub struct CHS
{
    cylinder: u8, /* 10 bits */
    head: u8, /* 8 bit */
    sector: u8, /* 6 bits */
}

pub struct PartionTableEntry
{
    pub boot: u8,
    start_chs: CHS,
    pub part_type: u8,
    end_chs: CHS,
    pub start_sector: u32,
    pub part_size: u32,
}


pub struct PartionTable
{
    pub part0: PartionTableEntry,
    pub part1: PartionTableEntry,
    pub part2: PartionTableEntry,
    pub part3: PartionTableEntry,
}

pub struct MasterBoostRecord
{
    pub code_area: [u8; 446],
    pub mpt: PartionTable,
    pub brs: u16, /* Boot Record signature */
}

fn u8tou64(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u64
{
    return (a as u64) << 56 | (b as u64) << 48 | (c as u64) << 40 | (d as u64) << 32 |
           (e as u64) << 24 | (f as u64) << 16 | (g as u64) << 8 | (h as u64); 
}

fn u8tou32(a: u8, b: u8, c: u8, d: u8) -> u32
{
    return (a as u32) << 24 | (b as u32) << 16 | (c as u32) << 8 | (d as u32); 
}

fn u8tou16(a: u8, b: u8) -> u16
{
    return (a as u16) << 8 | (b as u16);
}

fn u8stou64(slice: &[u8]) -> u64
{
    return u8tou64(slice[7], slice[6], slice[5], slice[4], 
                   slice[3], slice[2], slice[1], slice[0]);  
}

fn u8stou32(slice: &[u8]) -> u32
{
    return u8tou32(slice[3], slice[2], slice[1], slice[0]);
}

fn u8stou16(slice: &[u8]) -> u16
{
    return u8tou16(slice[1], slice[0]);
}

impl MasterBoostRecord
{
    pub fn new(sdio: RPISDIO) -> Self
    {
        let mut mbr_buf: [u8; 512] = sdio.readblock(0);
        let mbr = MasterBoostRecord {
            code_area: [0; 446],
            mpt: PartionTable {
                part0: PartionTableEntry {
                    boot: mbr_buf[446],
                    start_chs: CHS {
                        cylinder: mbr_buf[447],
                        head: mbr_buf[448],
                        sector: mbr_buf[449]
                    },
                    part_type: mbr_buf[450],
                    end_chs: CHS {
                        cylinder: mbr_buf[451],
                        head: mbr_buf[452],
                        sector: mbr_buf[453]
                    },
                    start_sector: u8stou32(&mbr_buf[454..458]),
                    part_size: u8stou32(&mbr_buf[458..462]),
                },
                part1: PartionTableEntry {
                    boot: mbr_buf[462],
                    start_chs: CHS {
                        cylinder: mbr_buf[463],
                        head: mbr_buf[464],
                        sector: mbr_buf[465]
                    },
                    part_type: mbr_buf[466],
                    end_chs: CHS {
                        cylinder: mbr_buf[467],
                        head: mbr_buf[468],
                        sector: mbr_buf[469]
                    },
                    start_sector: u8stou32(&mbr_buf[470..474]),
                    part_size: u8stou32(&mbr_buf[474..478]),                   
                },
                part2: PartionTableEntry {
                    boot: mbr_buf[478],
                    start_chs: CHS {
                        cylinder: mbr_buf[479],
                        head: mbr_buf[480],
                        sector: mbr_buf[481]
                    },
                    part_type: mbr_buf[482],
                    end_chs: CHS {
                        cylinder: mbr_buf[483],
                        head: mbr_buf[484],
                        sector: mbr_buf[485]
                    },
                    start_sector: u8stou32(&mbr_buf[486..490]),
                    part_size: u8stou32(&mbr_buf[490..494]),
                },
                part3: PartionTableEntry {
                    boot: mbr_buf[494],
                    start_chs: CHS {
                        cylinder: mbr_buf[495],
                        head: mbr_buf[496],
                        sector: mbr_buf[497]
                    },
                    part_type: mbr_buf[498],
                    end_chs: CHS {
                        cylinder: mbr_buf[499],
                        head: mbr_buf[500],
                        sector: mbr_buf[501]
                    },
                    start_sector: u8stou32(&mbr_buf[502..506]),
                    part_size: u8stou32(&mbr_buf[506..510]),
                }
            },
            brs: u8stou16(&mbr_buf[510..512])
        };
        mbr_buf[..446].copy_from_slice(&mbr.code_area);
        return mbr;
    }

    pub fn dump(&self)
    {
        use crate::uart::*;
        uart_print!("Part0:\n"); 
        uart_print!("\tboot:\t0x{:X}\n", self.mpt.part0.boot);
        uart_print!("\ttype:\t0x{:X}\n", self.mpt.part0.part_type);
        uart_print!("\tstart:\t0x{:X}\n", self.mpt.part0.start_sector);
        uart_print!("\tsize:\t0x{:X}\n", self.mpt.part0.part_size);
        uart_print!("Part1: \n"); 
        uart_print!("\tboot:\t0x{:X}\n", self.mpt.part1.boot);
        uart_print!("\ttype:\t0x{:X}\n", self.mpt.part1.part_type);
        uart_print!("\tstart:\t0x{:X}\n", self.mpt.part1.start_sector);
        uart_print!("\tsize:\t0x{:X}\n", self.mpt.part1.part_size);
        uart_print!("Part2: \n"); 
        uart_print!("\tboot:\t0x{:X}\n", self.mpt.part2.boot);
        uart_print!("\ttype:\t0x{:X}\n", self.mpt.part2.part_type);
        uart_print!("\tstart:\t0x{:X}\n", self.mpt.part2.start_sector);
        uart_print!("\tsize:\t0x{:X}\n", self.mpt.part2.part_size);
        uart_print!("Part3: \n"); 
        uart_print!("\tboot:\t0x{:X}\n", self.mpt.part3.boot);
        uart_print!("\ttype:\t0x{:X}\n", self.mpt.part3.part_type);
        uart_print!("\tstart:\t0x{:X}\n", self.mpt.part3.start_sector);
        uart_print!("\tsize:\t0x{:X}\n", self.mpt.part3.part_size);
    }
}