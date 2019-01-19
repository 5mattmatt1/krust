/*
 * Convert the code here at: https://github.com/brianwiddas/pi-baremetal/blob/master/memory.c
 * to Rust. Then maybe figure out why things need to be done this way.
 * It is neccessary for a working gpu...
 */

// Maybe this doesn't have to be static?
const PAGETABLE: *const u32 = mem_p2v(0x4000) as *const u32;

const fn mem_p2v(paddr: usize) -> usize
{
    return paddr + 0x80000000;
}

/* Maybe go for usize? */
pub unsafe fn mem_v2p(vaddr: usize) -> usize
{
    let pt_data: usize;
    let cpt_data: usize;
    let mut paddr: usize;

    pt_data = PAGETABLE.offset((vaddr >> 20) as isize) as usize;
    if (pt_data & 3) == 0 || (pt_data & 3) == 3
    {
        /* Nothing mapped */
        return 0xFFFFFFFF;
    }

    if (pt_data & 3) == 2
    {
        /* (Super)Selection */
        paddr = pt_data & 0xFFF00000;

        if (pt_data & (1 << 18)) != 0
        {
            /* 16 MB Supersection */
            paddr += vaddr & 0x00FFFFFF;
        } else
        {
            /* 1 MB Section */
            paddr += vaddr & 0x000FFFFF;
        }
        return paddr;
    }

    /* Coarse page table */
    cpt_data = *(((0x80000000 + (pt_data & 0xFFFFFC00)) as *const usize).offset(((vaddr >> 12) & 0xFF) as isize));
    if (cpt_data & 3) == 0
    {
        /* Nothing mapped */
        return 0xFFFFFFFF;
    }

    if (cpt_data & 2) != 0
    {
        /* Small (4k) page */
        return (cpt_data & 0xFFFFF000) + (vaddr & 0xFFF);
    }

    return (cpt_data & 0xFFFF0000) + (vaddr & 0xFFFF);
}