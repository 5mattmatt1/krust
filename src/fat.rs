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

pub struct BiosParameterBlock
{
    oem_identifier: u64,
    bps: u16, /* bytes per sector */
    spc: u8, /* sectors per cluster */
    pub num_rsec: u16,
    pub num_fat: u8,
    num_dirent: u16,
    ttl_sectors: u16,
    med_desc_type: u8,
    num_sec_pfat: u16,
    num_sec_trck: u16,
    num_heads: u16,
    num_hsec: u32,
    lrg_sec_cnt: u32,
}

/* FAT32 */
/* Extended Boot Parameter */
pub struct ExtBiosParameterBlock
{
    pub num_sec_pfat: u32,
    flags: u16,
    fat_ver: u16,
    root_cluster: u32,
    fs_info_secn: u16,
    bck_bsec_secn: u16,
    reserved: [u8; 12],
    drive_num: u8,
    flags_nt: u8,
    signature: u8,
    volume_id: u32,
    volume_label: [u8; 11],
    sys_identifier: u64,
    /* Don't include boot code */
    bp_signature: u16, /* 0xAA55 */
}

enum FileAttribute
{
    ReadOnly = 0x01,
    Hidden = 0x02,
    System = 0x04,
    VolumeId = 0x08,
    Directory = 0x10,
    Archive = 0x20,
    Lfe = 0x0F /* ReadOnly | Hidden | System | VolumeId */
}

pub struct DirEnt
{
    filename: [u8; 11],
    file_attr: u8,
    win_nt: u8,
    create_time_tsec: u8, /* 0-199 */
    create_time: u16,
    create_date: u16,
    access_date: u16,
    high_cluster: u16,
    modif_time: u16,
    modif_date: u16,
    low_cluster: u16,
    file_size: u32,
}

struct LongFilenameEntry
{
    order: u8,
    first_chars: [u16; 5],
    attr: u8,
    le_type: u8,
    checksum: u8,
    next_chars: [u16; 6],
    always_zero: u16,
    final_chars: [u16; 2],
}

struct Node
{
    lba: usize,
    offset: usize,
}

struct Inode
{
    has_lfe: bool,
    lfe_node: Node,
    dir_ent_node: Node
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

impl BiosParameterBlock
{
    pub fn new(sector: [u8; 512]) -> Self
    {
        return BiosParameterBlock {
            oem_identifier: u8stou64(&sector[3..11]),
            bps: u8stou16(&sector[11..13]),
            spc: sector[13],
            num_rsec: u8stou16(&sector[14..16]),
            num_fat: sector[16],
            num_dirent: u8stou16(&sector[17..19]),
            ttl_sectors: u8stou16(&sector[19..21]),
            med_desc_type: sector[21],
            num_sec_pfat: u8stou16(&sector[22..24]),
            num_sec_trck: u8stou16(&sector[24..26]),
            num_heads: u8stou16(&sector[26..28]),
            num_hsec: u8stou32(&sector[28..32]),
            lrg_sec_cnt: u8stou32(&sector[32..36]),
        };
    }

    pub fn dump(&self)
    {
        use crate::uart::*;
        uart_print!("BiosParameterBlock: (FAT32)\n");
        uart_print!("\tOEM:\t0x{:X}\n", self.oem_identifier);
        uart_print!("\tBytes per sector:\t0x{:X}\n", self.bps);
        uart_print!("\tSectors per cluster:\t0x{:X}\n", self.spc);
        uart_print!("\t# Reserved Sectors:\t0x{:X}\n", self.num_rsec);
        uart_print!("\t# FAT's:\t0x{:X}\n", self.num_fat);
        uart_print!("\t# Directories:\t0x{:X}\n", self.num_dirent);
        uart_print!("\tTotal Sectors:\t0x{:X}\n", self.ttl_sectors);
        uart_print!("\t# of Hidden Sectors:\t0x{:X}\n", self.num_hsec);
        uart_print!("\t# of Sectors (Large):\t0x{:X}\n", self.lrg_sec_cnt);
    }
}

/* FAT32 */
impl ExtBiosParameterBlock
{
    pub fn new(sector: [u8; 512]) -> Self
    {
        let mut ebpb = ExtBiosParameterBlock {
            num_sec_pfat: u8stou32(&sector[36..40]),
            flags: u8stou16(&sector[40..42]),
            fat_ver: u8stou16(&sector[42..44]),
            root_cluster: u8stou32(&sector[44..48]),
            fs_info_secn: u8stou16(&sector[48..50]),
            bck_bsec_secn: u8stou16(&sector[50..52]),
            reserved: [0; 12],
            drive_num: sector[64],
            flags_nt: sector[65],
            signature: sector[66],
            volume_id: u8stou32(&sector[67..71]),
            volume_label: [0; 11],
            sys_identifier: u8stou64(&sector[82..90]),
            bp_signature: u8stou16(&sector[510..512])
        };

        ebpb.volume_label.clone_from_slice(&sector[71..82]);
        return ebpb;
    }

    pub fn dump(&self)
    {
        use crate::uart::*;
        use core::str::from_utf8;
        uart_print!("Extended BIOS Parameter Block (FAT32): \n");
        uart_print!("\t# of Sectors per FAT:\t0x{:X}\n", self.num_sec_pfat);
        uart_print!("\tFlags:\t0x{:X}\n", self.flags);
        uart_print!("\tFAT Version:\t0x{:X}\n", self.fat_ver);
        uart_print!("\tRoot Cluster:\t0x{:X}\n", self.root_cluster);
        uart_print!("\tFSInfo Sector#:\t0x{:X}\n", self.fs_info_secn);
        uart_print!("\tBackup Boot Sector #:\t0x{:X}\n", self.bck_bsec_secn);
        uart_print!("\tDrive #:\t0x{:X}\n", self.drive_num);
        uart_print!("\tSignature:\t0x{:X}\n", self.signature);
        uart_print!("\tVolume ID:\t0x{:X}\n", self.volume_id);
        uart_print!("\tVolume Label:\t{}\n", from_utf8(&self.volume_label).unwrap());
        uart_print!("\tSystem Identifier:\t0x{:X}\n", self.sys_identifier);
        uart_print!("\tBoot partition signature:\t0x{:X}\n", self.bp_signature);
    }
}

impl DirEnt
{
    pub fn new(block: &[u8]) -> Self
    {
        let mut dir_ent = DirEnt {
            filename: [0; 11],
            file_attr: block[11],
            win_nt: block[12],
            create_time_tsec: block[13],
            create_time: u8stou16(&block[14..16]),
            create_date: u8stou16(&block[16..18]),
            access_date: u8stou16(&block[18..20]),
            high_cluster: u8stou16(&block[20..22]),
            modif_time: u8stou16(&block[22..24]),
            modif_date: u8stou16(&block[24..26]),
            low_cluster: u8stou16(&block[26..28]),
            file_size: u8stou32(&block[28..32]),
        };

        dir_ent.filename.clone_from_slice(&block[0..11]);
        return dir_ent;
    }

    pub fn dump(&self)
    {
        use crate::uart::*;
        use core::str::from_utf8;
        uart_print!("Directory Entity (FAT32):\n");
        uart_print!("\tFilename:\t{}\n", from_utf8(&self.filename).unwrap());
        uart_print!("\tAttributes:\t0x{:X}\n", self.file_attr);
        uart_print!("\tCreate time:\t"); Self::time_str(self.create_time);
        uart_print!("\tCreate date:\t"); Self::date_str(self.create_date);
        uart_print!("\tAccess date:\t"); Self::date_str(self.access_date);
        uart_print!("\tModification time:\t"); Self::time_str(self.modif_time);
        uart_print!("\tModification date:\t"); Self::date_str(self.modif_date);
        uart_print!("\tHigh cluster:\t0x{:X}\n", self.high_cluster);
        uart_print!("\tLow cluster:\t0x{:X}\n", self.low_cluster);
        uart_print!("\tFile size:\t0x{:X}\n", self.file_size);
    }

    pub fn time_str(time: u16)
    {
        use crate::uart::*;
        const HOUR_MASK: u16 = 0xF800;
        const MINUTE_MASK: u16 = 0x07E0;
        const SECOND_MASK: u16 = 0x1F;
        const HOUR_SHIFT: u16 = 11;
        const MINUTE_SHIFT: u16 = 5;
        const SECOND_SHIFT: u16 = 0;
        uart_print!("{}:{}:{}\n", (time & HOUR_MASK) >> HOUR_SHIFT,
                                  (time & MINUTE_MASK) >> MINUTE_SHIFT,
                                  (time & SECOND_MASK) >> SECOND_SHIFT);
    }

    // ISO 8601
    pub fn date_str(date: u16)
    {
        use crate::uart::*;
        const YEAR_MASK: u16 = 0xFE00;
        const MONTH_MASK: u16 = 0x01E0;
        const DAY_MASK: u16 = 0x001F;
        const YEAR_SHIFT: u16 = 9;
        const MONTH_SHIFT: u16 = 5;
        const DAY_SHIFT: u16 = 0;
        uart_print!("{}/{}/{}\n", 1980 + ((date & YEAR_MASK) >> YEAR_SHIFT),
                                    (date & MONTH_MASK) >> MONTH_SHIFT,
                                    (date & DAY_MASK) >> DAY_SHIFT);
    }
}