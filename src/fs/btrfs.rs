// https://btrfs.wiki.kernel.org/index.php/Data_Structures

// #![crate_name = "doc"]


/* 
 * Might be useful for documentation so that people 
 * know that the data structures use little endian
 * instead of host order. 
 */
// type leu64 = u64;
// type leu32 = u32;
// type leu16 = u16;
// type leu8 = u8;

pub type UUID = [u8; 16];

/// Defines the location, properties, and usage of a block group.
/// It is associated with the BLOCK_GROUP_ITEM. This structure is
/// never used outside this item.
pub struct BlockGroupItem
{
    /// The space used in this block group
    pub used: u64,
    /// The objectid of the chunk backing this block group
    pub chunk_objectid: u64,
    flags: u64,
}

/// The type of storage this block group offers. SYSTEM chunks cannot be mixed, 
/// but DATA and METADATA chunks can be mixed.
#[repr(u64)]
pub enum BlockGroupItemAllocationType
{
    Data = 0x1,
    System = 0x2,
    Metadata = 0x4,
}

/// The allocation policy this block group implements. 
/// Only one of the following flags may be set in any single block group. 
/// It is not possible to combine policies to create nested RAID levels 
/// beyond the RAID-10 support offered below. If no flags are specified, 
/// the block group is not replicated beyond a single, unstriped copy.
#[repr(u64)]
pub enum BlockGroupItemReplicationPolicy
{
    /// Striping (RAID-0)
    Raid0 = 0x08,
    /// Mirror on a separate device (RAID-1)
    Raid1 = 0x10,
    /// Mirror on a single device
    Dup = 0x20,
    /// Striping and mirroring (RAID-10)
    Raid10 = 0x40,
    /// Parity striping with single-disk fault tolerance (RAID-5)
    Raid5 = 0x80,
    /// Parity striping with double-disk fault tolerance (RAID-6)
    Raid6 = 0x100,
}

/// This structure contains the mapping from a virtualized usable byte 
/// range within the backing storage to a set of one or more stripes on individual backing devices. 
/// In addition to the mapping, hints on optimal I/O parameters for this chunk. 
/// It is associated with CHUNK_ITEM items.
/// Although the structure definition only contains one stripe member, 
/// CHUNK_ITEM items contain as many struct btrfs_stripe structures as specified in the num_stripes and sub_stripes fields.
pub struct Chunk
{
    /// Size of this chunk in bytes.
    pub length: u64,
    /// Objectid of the root referencing this chunk; Always EXTENT_ROOT.
    pub owner: u64,
    /// Replication stripe length.
    pub stripe_len: u64,
    /// Uses the same flags as btrfs_block_group_item.
    pub flags: u64,
    /// Optimal I/O alignment for this chunk.
    pub io_align: u32,
    /// Optimal I/O width for this chunk.
    pub io_width: u32,
    /// Minimal I/O size for this chunk.
    pub sector_size: u32,
    /// Number of replication stripes.
    pub num_stripes: u16,
    /// Number of replication sub-stripes (used only for RAID-10).
    pub stripe: Stripe,
}

/// Start of a variable-length sequence of [currently 4-byte] checksums,
/// one for each 4k of data, starting at the offset field of the key.
pub type CSumItem = [u8; 4];

/// This structure is used to map physical extents on an individual backing device to a chunk. 
/// This extent may be the only one for a particular chunk or one of several.
/// It is associated with the DEV_ITEM item. This structure is never used outside of this item.
pub struct DevExtent
{
    /// Objectid of the CHUNK_TREE that owns this extent.
    /// Always BTRFS_CHUNK_TREE_OBJECTID. 
    /// It is unclear whether the chunk_tree and chunk_tree_uuid fields 
    /// are a remnant of an early design or included for potential future expansion.
    pub chunk_tree: u64,
    /// Objectid of the CHUNK_ITEM that references this extent.
    /// In practice, it will always be BTRFS_FIRST_CHUNK_TREE_OBJECTID
    pub chunk_objectid: u64,
    /// Offset of the CHUNK_ITEM that references this extent.
    pub chunk_offset: u64,
    /// Length of this extent, in bytes.
    pub length: u64,
    /// UUID of the CHUNK_TREE that owns this extent.
    pub chunk_tree_uuid: [u8; 16],
}

/// Represents a complete block device.
/// devid 
///     Matches the devid in the filesystem's list of struct btrfs_devices.
pub struct DevItem
{
    /// the internal btrfs device id
    pub devid: u64,
    /// size of the device
    pub total_bytes: u64,
    /// bytes used
    pub bytes_used: u64,
    /// optimal io alignment for this device
    pub io_align: u32,
    /// optimal io width for this device
    pub io_width: u32,
    /// minimal io size for this device
    pub sector_size: u32,
    /// type and info about this device
    pub dev_type: u64,
    /// expected generation for this device
    pub generation: u64,
    /// starting byte of this partition on the device,
    /// to allow for stripe alignment in the future
    pub start_offset: u64,
    /// grouping information for allocation decisions
    pub dev_group: u32,
    /// seek speed 0-100 where 100 is fastest
    pub seek_speed: u8,
    /// bandwidth 0-100 where 100 is fastest
    pub bandwidth: u8,
    /// btrfs generated uuid for this device
    pub uuid: [u8; 16],
    /// uuid of FS who owns this device
    pub fsid: [u8; 16],
}

/// This device item holds various statistics about a device. 
/// This item type is contained only in the DEV_TREE
pub struct DevStatsItem
{
    /// Number of times we got EIO or EREMOTEIO 
    /// from lower layers while performing writes
    pub write_errs: u64,
    /// Number of times we got EIO or EREMOTEIO 
    /// from lower layers while performing reads
    pub read_errs: u64,
    /// Number of times we got EIO or EREMOTEIO 
    /// from lower layers while performing data flushes
    pub flush_errs: u64,
    /// checksum error, bytenr error or contents is illegal: 
    /// this is an indication that the block was damaged 
    /// during read or write, or written to wrong location
    /// or read from wrong location
    pub corruption_err: u64,
    /// an indication that blocks have not been written
    pub generation_err: u64, 
}

/// This item holds necessary information to resume a device
/// replace operation following a crash/suspend. 
/// It is contained in DEV_TREE
pub struct DevReplaceItem
{
    /// Device id of the source device
    pub src_devid: u64,
    cursor_left: u64,
    cursor_right: u64,
    cont_reading_from_srcdev_mode: u64,
    /// State of replace operation.
    pub replace_state: IoctlDevReplaceState,
    time_started: u64,
    time_stopped: u64,
    /// expected generation for this device
    pub num_uncorrectable_read_errors: u64,
}

pub enum IoctlDevReplaceState
{
    Started,
}

/// This structure represents the header for a directory entry 
/// item used for both standard user-visible directories and 
/// internal directories used to manage named extended attributes.
/// It is associated with the DIR_ITEM and XATTR_ITEM items. 
/// This structure is not used outside of these items.
/// It is immediately followed by the name. If it represents an 
/// extended attribute, the attribute data immediately 
/// follows the name.
pub struct DirItem
{
    /// Key for the INODE_ITEM or ROOT_ITEM associated with this entry. 
    /// Unused and zeroed out when the entry describes an extended attribute.
    pub location: DiskKey,
    /// transid of the transaction that created this entry.
    pub transid: u64,
    /// Length of the extended attribute associated with this entry.
    /// 0 for standard directories.
    pub data_len: u16,
    /// Length of directory entry name
    pub name_len: u16,
    pub file_type: FileType,

}

/// These directory entry types use the same values as the 
/// d_type field in struct dirent documented in the readdir(3)
/// manual page.
pub enum FileType
{
    /// The target object's type is unknown. 
    /// Indicates corruption if used.
    Unknown = 0,
    /// The target object is an INODE_ITEM representing a regular file.
    RegFile = 1,
    /// The target object is an INODE_ITEM representing a directory
    /// or a ROOT_ITEM that will be presented as a directory.
    Dir = 2,
    /// The target object is an INODE_ITEM representing a character device node.
    ChrDev = 3,
    /// The target object is an INODE_ITEM representing a block device node.
    BlkDev = 4,
    /// The target object is an INODE_ITEM representing a FIFO device node.
    Fifo = 5,
    /// The target object is an INODE_ITEM representing a socket device node.
    Sock = 6,
    /// The target object is an INODE_ITEM representing a symbolic link.
    Symlink = 7,

    /// This value is used on-disk and internally but is not user-visible.
    /// The entry is an XATTR_ITEM.
    Xattr = 8,
}

/// Every tree block (leaf or node) starrts with this header
pub struct Header
{
    /// virtual address of block
    pub bytenr: u32,
    /// the object id of the tree this block belongs to, 
    /// for example BTRFS_ROOT_TREE_OBJECTID
    pub owner: u8,
}

/// The btrfs_key is one of the fundamental btrfs data structures. 
/// Every item in every tree in the file system is located using 
/// its key. The btrfs_key can be more accurately described as a 
/// 3-tuple used to locate any item in any tree in the file system.
/// btrfs_key objects only exists in memory and is in CPU byte order. 
pub struct Key
{
    /// The object identifier for this item
    objectid: u64,
    /// The type of the item this key describes
    key_type: u8,
    /// More accurately described as "third component." 
    /// It is literally an offset only in some contexts.
    offset: u64,
}

/// btrfs_disk_key is identical to btrfs_key except that objectid 
/// and offset are in little endian (disk) byte order and are 
/// part of the file system on-disk format.
pub type DiskKey = Key;

pub struct INodeItem
{
    /// This structure contains the information typically 
    /// associated with a UNIX-style inode's stat(2) data. 
    /// It is associated with the INODE_ITEM.
    generated: u64,
    transid: u64,
    size: u64,
    nbytes: u64,
    block_group: u64,
    nlink: u32,
    uid: u32,
    gid: u32,
    mode: u32,
    rdev: u64,
    flags: u64,
    sequence: u64,
    reserved: [u64; 4],
    atime: Timespec,
    ctime: Timespec,
    mtime: Timespec,
    otime: Timespec
}

pub enum INodeFlags
{
    /// Do not perform checksum operations on this inode.
    NoDataSum = 0x1,
    /// Do not perform CoW for data extents on this inode when the reference count is 1.
    NoDataCow = 0x2,
    /// Inode is read-only regardless of UNIX permissions or ownership.
    /// This bit is still checked and returns EACCES but there is no way to set it. That suggests that it has been superseded by BTRFS_INODE_IMMUTABLE.
    ReadOnly = 0x4,
    /// Do not compress this inode.
    /// This flag may be changed by the kernel as compression ratios change. If the compression ratio for data associated with an inode becomes undesirable, this flag will be set. It may be cleared if the data changes and the compression ratio is favorable again.
    NoCompress = 0x8,
    /// Inode contains preallocated extents. This instructs the kernel to attempt to avoid CoWing those extents.
    Prealloc = 0x10,
    /// Operations on this inode will be performed synchronously.
    /// This flag is converted to a VFS-level inode flag but is not handled anywhere.
    ISync = 0x20,
    /// Inode is read-only regardless of UNIX permissions or ownership. Attempts to modify this inode will result in EPERM being returned to the user.
    Immutable = 0x40,
    /// This inode is append-only.
    Append = 0x80,
    /// This inode is not a candidate for dumping using the dump(8) program.
    /// This flag will be accepted on all kernels but is not implemented
    NoDump = 0x100,
    /// Do not update atime when this inode is accessed.
    NoATime = 0x200,
    /// Operations on directory operations will be performed synchronously.
    /// This flag is converted to a VFS-level inode flag but is not handled anywhere.
    DirSync = 0x400,
    /// Compression is enabled on this inode.
    Compress = 0x800,
}

pub struct Timespec 
{
    pub seconds: i64,
    pub nanoseconds: i32,
}

/// This structure holds defines the the root of a btree. 
/// It is associated with the ROOT_ITEM type. 
/// This structure is never used outside of this item.
pub struct RootItem
{
    /// Several fields are initialized but only flags 
    /// is interpreted at runtime.
    /// generation=1, size=3,nlink=1, nbytes=<leafsize>, mode=040755
    /// flags depends on kernel version, see below.
    pub inode: INodeItem,
    /// transid of the transaction that created this root.
    pub generation: u64,
    /// For file trees, the objectid of the root directory 
    /// in this tree (always 256). Otherwise, 0.
    pub root_dirid: u64,
    /// The disk offset in bytes for the root node of this tree.
    pub bytnr: u64,
    /// Unused. Always 0.
    pub byte_limit: u64,
    /// Unused.
    pub bytes_used: u64,
    /// The last transid of the transaction that created a snapshot of this root.
    pub last_snapshot: u64,
    
    pub flags: u64,
    /// Originally indicated a reference count. 
    /// In modern usage, it is only 0 or 1.
    pub refs: u32,
    /// Contains key of last dropped item during subvolume removal or relocation. Zeroed otherwise.
    pub drop_progress: DiskKey,
    /// The tree level of the node described in drop_progress.
    pub drop_level: u8,
    /// The height of the tree rooted at bytenr.
    pub level: u8,
    /// If equal to generation, indicates validity of the following fields.
    /// If the root is modified using an older kernel, 
    /// this field and generation will become out of sync. 
    /// This is normal and recoverable.
    pub generation_v2: u64,
    /// This subvolume's UUID.
    pub uuid: [u8; 16],
    /// The parent's UUID (for use with send/receive).
    pub parent_uuid: [u8; 16],
    /// The received UUID (for used with send/receive).
    pub received_uuid: [u8; 16],
    /// The transid of the last transaction that modified this tree, 
    /// with some exceptions (like the internal caches or relocation).
    pub ctransid: u64,
    /// The transid of the transaction that created this tree.
    pub otransid: u64,
    /// The transid for the transaction that sent this subvolume. 
    /// Nonzero for received subvolume.
    pub stransid: u64,
    /// The transid for the transaction that received this subvolume. 
    /// Nonzero for received subvolume.
    pub rtransid: u64,
    /// Timestamp for ctransid.
    pub ctime: Timespec,
    /// Timestamp for otransid.
    pub otime: Timespec,
    /// Timestamp for stransid.
    pub stime: Timespec,
    /// Timestamp for rtransid.
    pub rtime: Timespec,
    reserved: [u64; 8],
}

/// This structure is used to define the backing device storage that compose a btrfs chunk. 
/// It is associated with the CHUNK_ITEM item. This structure is never used outside of this item.
pub struct Stripe
{
    /// Device ID that contains this stripe.
    pub devid: u64,
    /// Location of the start of the stripe, in bytes.
    /// Size is determined by the stripe_len field in struct btrfs_chunk.
    pub offset: u64,
    /// UUID of the device that contains this stripe. Used to confirm that the correct device has been retrieved.
    pub dev_uuid: [u8; 16],
}

/// The primary superblock is located at 0x1 0000 (6410 KiB). 
/// Mirror copies of the superblock are located at physical addresses
/// 0x400 0000 (6410 MiB), 0x40 0000 0000 (25610 GiB), 
/// and 0x4 0000 0000 0000 (1 PiB), if these locations are valid. 
/// btrfs normally updates all superblocks, 
/// but in SSD mode it will update only one at a time. 
/// The superblock with the highest generation is used when reading.
/// Note that btrfs only recognizes disks with a valid 
/// 0x1 0000 superblock; otherwise, there would be confusion 
/// with other filesystems.
pub struct SuperBlock
{
    /// Checksum of everything past this field (from 20 to 1000)
    pub csum: [u8; 20],
    /// FS UUID
    pub fsid: [u8; 10],
    /// physical address of this block (different for mirrors)
    pub bytenr: u64,
    flags: u64,
    /// magic ("_BHRfS_M")
    pub magc: u64,
    /// generation
    pub generation: u64,
    /// logical address of the root tree root
    pub root: u64,
    /// logical address of the chunk tree root
    pub chunk_root: u64,
    /// logical address of the log tree root
    pub log_root: u64,
    // log_root_transid
    pub log_root_transid: u64,
    /// total_bytes
    pub total_bytes: u64,
    /// bytes_used
    pub bytes_used: u64,
    /// root_dir_objectid (usually 6)
    pub root_dir_objectid: u64,
    /// sector_size
    pub sector_size: u64,
    /// node_size
    pub node_size: u64,
    /// Unused
    pub leaf_size: u64,
    /// stripe_size
    pub stripe_size: u64,
    /// sys_chunk_array_size
    pub sys_chunk_array_size: u64,
    /// chunk_root_generation
    pub chunk_root_generation: u64,
    /// compat_flags
    pub compat_flags: u64,
    /// only implementations that support the flags can write to the filesystem
    pub compat_ro_flags: u64,
    ///  only implementations that support the flags can use the filesystem
    pub incompat_flags: u64,
    /// Btrfs currently uses the CRC32c little-endian hash function with seed -1.
    pub csum_type: u64,
    /// root_level
    pub root_level: u8,
    /// chunk_root_level
    pub chunk_root_level: u8,
    /// log_root_level
    pub log_root_level: u8,
    /// DEV_ITEM data for this device
    pub dev_item: DevItem,
    /// label (may not contain '/' or '\\')
    pub label: [u8; 100],
    /// cache_generation
    pub cache_generation: u64,
    /// uuid_tree_generation
    pub uuid_tree_generation: u64,
    /// reserved /* future expansion */
    pub reserved: [u64; 30],
    /// sys_chunk_array:(n bytes valid) Contains (KEY, CHUNK_ITEM) 
    /// pairs for all SYSTEM chunks. This is needed to bootstrap the 
    /// mapping from logical addresses to physical.
    pub sys_chunk_array: [u8; 800],
    // Contain super_roots (4 btrfs_root_backup)
    // super_roots: [RootBackup; NUM_BACKUP_ROOTS]
}