/// Not sure where these come from...
struct PageInfo
{
    tg: u8,
    /// Access permissions
    ap: u8,
    /// Read/Write
    /// 0 - Read/Write
    /// 1 - Read-only
    rw: u8,
    /// Accessed flag
    af: u8,
}

enum InnerCacheable
{
    /// Normal memory, Inner Non-cacheable
    No = 00,
    /// Normal memory, Inner Write-Back Write Allocate Cacheable
    WriteBackAlloc = 01,
    /// Normal memory, Inner Write-Through Cacheable
    WriteThrough = 10,
    /// Normal memory, Inner Write-Back no Write-Allocate Cacheable
    WriteBackNoAlloc = 0b11,
}

enum Shareability
{
    /// Non-shareable
    None = 0b00,
    /// Unpredictable, do not use
    Unpredictable = 0b01,
    /// Outer Shareable
    Outer = 0b10,
    /// Inner Shareable
    Inner = 0b11,
}

/// Unprivileged (EL0)
#[repr(u8)]
enum UAccessPerm
{
    None = 0b00,
    ReadWrite = 0b01,
    NoneB = 0b10,
    ReadOnly = 0b11,
}

/// Privileged Access Permissions (EL1/EL2/EL3)
#[repr(u8)]
enum PAccessPerm
{
    ReadWrite = 0b00,
    ReadWriteB = 0b01,
    ReadOnly = 0b10,
    ReadOnlyB = 0b11,
}

/// Granularity of a memory attribute
enum MAGranularity
{
    /// 4 KB
    Page = 0b11,
    /// 2 MB
    Block = 0b01,
}

/// Memory Attribute for a stage one block entry
struct MemoryAttribute
{
    /// Unprivileged eXecute Never
    uxn: u8,
    /// Privileged eXecute Never
    puxn: u8,
    /// Output block address
    oba: u32,
    /// Access flag
    af: u8,
    /// Shareable attribute
    sh: u8,
    // Access permission according to documentation,
    // but worthwhile to split into two fields [7:6]
    /// Access Privilege
    ap: u8,
    /// Read/Write ability
    rw: u8,
    /// Security bit,
    /// but only at EL3 and Secure EL1
    ns: u8,
    /// Index into the MAIR_ELn Register
    indx: u8,
    /// Granularity
    /// 0b11 - 4k Granule
    /// 0b01 - 2M Granule
    tg: u8,
}
/// Memory Attribute Indirection Register
/// MAIR_ELn

struct MemoryAttributesArray
{
    attr_idx0 : u8,
    attr_idx1 : u8,
    attr_idx2 : u8,
}

struct TCREntry
{
    /// Translation Granule
    /// indicates the smallest block of memory that can be
    /// independently mapped in the translation tables.
    /// 00 = 4KB
    /// 01 = 16KB
    /// 11 = 64KB
    tg: u8,
    /// Shareability
    sh: u8,
    /// Outer cacheability
    orgn: u8,
    /// Inner cacheability
    irgn: u8,
    /// Translation walk enable
    epd: u8,
    /// Table size?
    tsz: u8,
}

struct TranslationControlRegister
{
    tbi: u32,
    /// Intermediate Physical Address Size,
    /// controls the maximum output size.
    ips: u32,
    /// Translation Control entry for kernelspace
    entry1: TCREntry,
    /// Translation Control entry for userspace
    entry0: TCREntry,
}

pub fn init(_kpage_table: &[u8], _upage_table: &[u8])
{
    /* Thank you bztsrc */
    let mmfr: u64;
    unsafe { asm!("mrs $0, id_aa64mmfr0_el1" : "=r"(mmfr)) };
    if (mmfr & (0xF << 28) != 0) && ((mmfr & 0xF) < 1)
    {
        panic!("ERROR: 4k granule or 36 bit address space not supported\n");
    }

    asm!("msr mair_el1, $0" : "=r"());
    // asm!("msr tcr_el1, $0; isb" :"=r"());
    // tell the MMU where our translation tables are.
    // Userspace
    // asm!("msr ttbr0_el1, $0" : "=r"(upage_table));
    // Kernelspace
    // asm!("msr ttbr1_el1, $0" : "=r"(kpage_table));

    // Finally, toggle some bits in system control register to enable page translation
    // asm!("dsb ish; isb; mrs $0, sctlr_el1" : "=r"());
    // asm!("msr sctlr_el1, $0; isb" : "=r"());
    // return heap;
}