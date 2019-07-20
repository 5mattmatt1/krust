#![no_std]
#![no_main]
#![feature(asm)]

#[macro_use]
extern crate krust;

// fn test_sdhci_dma()
// {
//     use krust::utils::slice::convert_slice;
//     let mut block0: [u32; 128] = [0; 128];
//     sdio.dma_readblock(0, &mut block0);
//     let u8block0: &[u8];
//     unsafe 
//     {
//         u8block0 = convert_slice(&block0);
//     } 
// }

fn test_fat()
{
    use krust::mbr::*;
    use krust::sdhci::{SDIO, RPISDIO};
    use krust::fat::{BiosParameterBlock, ExtBiosParameterBlock, DirEnt};
    let mut sdio = RPISDIO::new();
    sdio.init();

    let mbr: MasterBoostRecord = MasterBoostRecord::new(sdio);
    mbr.dump();
    let block0 = sdio.readblock(mbr.mpt.part0.start_sector as usize);
    let bpb = BiosParameterBlock::new(block0);
    let ebpb = ExtBiosParameterBlock::new(block0);
    bpb.dump();
    ebpb.dump();
    let mut fat_root_lba = (ebpb.num_sec_pfat as u32 * bpb.num_fat as u32) + bpb.num_rsec as u32;
    fat_root_lba += mbr.mpt.part0.start_sector;
    // let fat_table_lba = mbr.mpt.part0.start_sector + bpb.num_rsec as u32;
    let fat_root = sdio.readblock(fat_root_lba as usize);
    DirEnt::new(&fat_root[..]).dump();
    DirEnt::new(&fat_root).dump();
    uart_println!("Allocation successful!");
}

fn test_entry() -> ! 
{
    test_fat();
    loop 
    {
        unsafe { asm!("nop") };
    }
}

raspi3_boot::entry!(test_entry);