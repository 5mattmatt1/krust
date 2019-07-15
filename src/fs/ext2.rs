pub struct SuperBlock
{
    /// Total number of inodes in file system
    pub total_inodes: u32,
    /// Total number of blocks in file system
    pub total_blocks: u32,
    /// Number of blocks reserved for superuser
    /// See uid_reserved_blocks
    pub reserved_blocks: u32,
    /// Total number of unallocated blocks
    pub unalloc_blocks: u32,
    /// Total number of unallocated inodes
    pub unalloc_inodes: u32,
    /// Block number of the block containing the superblock
    pub super_block: u32,
    /// log2 (block size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the block size)
    pub block_size: u32,
    /// log2 (fragment size) - 10. (In other words, the number to shift 1,024 to the left by to obtain the fragment size)
    pub fragment_size: u32,
    /// Number of blocks in each block group
    pub blocks_per_group: u32,
    /// Number of fragments in each block group
    pub fragments_per_group: u32,
    /// Number of inodes in each block group
    pub inodes_per_group: u32,
    /// Last mount time (POSIX time)
    pub mount_time: u32,
    /// Last written time (POSIX time)
    pub write_time: u32,
    /// Number of times the volume has been mounted since 
    /// its last consistency check (fsck)
    pub fsck_mounts: u16,
    /// Number of mounts allowed before a consistency check
    /// (fsck) must be done
    pub fsck_mounts_allowed: u16,
    /// Ext2 signature (0xef53), used to help confirm 
    /// the presence of Ext2 on a volume
    pub signature: u16,
    /// File System state
    pub fs_state: FileSystemState,
    /// What to do when an error is detected
    pub err_handle: ErrorHandle,
    /// Minor portion of version
    pub min_ver: u16,
    /// POSIX time of last consistency check (fsck)
    pub fsck_time: u32,
    /// Interval (in POSIX time) between forced consistency checks (fsck)
    pub fsck_time_allowed: u32,
    /// Operating system ID from which the filesystem
    /// on this volume was created (see below)
    pub os_id: u32,
    /// Major portion of version
    pub maj_ver: u32,
    /// User ID that can use reserved blocks
    pub uid_reserved_blocks: u16,
    /// Group ID that can use reserved blocks
    pub gid_reserved_blocks: u16,
}

#[repr(u16)]
pub enum FileSystemState
{
    Clean = 1,
    Error = 2,
}

#[repr(u16)]
pub enum ErrorHandle
{
    /// Ignore the error (continue on)
    Ignore = 1,
    /// Remount file system as read-only
    RemountReadOnly = 2,
    /// Kernel panic
    Panic = 3,
}

#[repr(u32)]
pub enum OSId
{
    Linux = 0,
    GNUHurd = 1,
    /// MASIX (an operating system developed by Rémy Card, one of the developers of ext2)
    Masix = 2,
    FreeBSD = 3,
    /// Other "Lites" (BSD4.4-Lite derivatives such as NetBSD, OpenBSD, XNU/Darwin, etc.)
    Lite = 4,
}

/// These fields are only present if Major version 
/// (specified in the base superblock fields), 
/// is greater than or equal to 1.
pub struct ExtSuperBlock
{

}

/// A Block Group Descriptor contains information regarding 
/// where important data structures for that block group are located.
pub struct BlockGroupDescriptor
{

}

pub struct INode
{
    pub types_and_permissions: u16,
    /// User ID
    pub uid: u16,
    /// Lower 32 bits of size in bytes
    pub li_size_bytes: u32,
    /// Last Access Time (POSIX time)
    pub atime: u32,
    /// Creation Time (POSIX time)
    pub ctime: u32,
    /// Last Modification time (POSIX time)
    pub mtime: u32,
    /// Deletion time (POSIX time)
    pub dtime: u32,
    /// Group ID
    pub gid: u16,
    /// Count of hard links (directory entries) to this inode. 
    /// When this reaches 0, the data blocks are marked as unallocated.
    pub hard_link_count: u16,
    /// Count of disk sectors (not Ext2 blocks) in use by this inode,
    /// not counting the actual inode structure nor directory entries linking to the inode.
    pub disk_sectors: u32,
    /// Flags
    pub flags: u32,
    /// Technically OS specific, GNU HURD does something? with it
    pub _reserved: u32,
    /// Direct Block Pointer 0
    pub direct_block_ptr0: u32,
    /// Direct Block Pointer 1
    pub direct_block_ptr1: u32,
    /// Direct Block Pointer 2
    pub direct_block_ptr2: u32,
    /// Direct Block Pointer 3
    pub direct_block_ptr3: u32,
    /// Direct Block Pointer 4
    pub direct_block_ptr4: u32,
    /// Direct Block Pointer 5
    pub direct_block_ptr5: u32,
    /// Direct Block Pointer 6
    pub direct_block_ptr6: u32,
    /// Direct Block Pointer 7
    pub direct_block_ptr7: u32,
    /// Direct Block Pointer 8
    pub direct_block_ptr8: u32,
    /// Direct Block Pointer 9
    pub direct_block_ptr9: u32,
    /// Direct Block Pointer 10
    pub direct_block_ptr10: u32,
    /// Direct Block Pointer 11
    pub direct_block_ptr11: u32,
    /// Singly Indirect Block Pointer 
    /// (Points to a block that is a list of block pointers to data)
    pub single_indirect_block_ptr: u32,
    ///
    pub double_indirect_block_ptr: u32,
    ///
    pub triple_indirect_block_ptr: u32,
    /// Generation number (Primarily used for NFS)
    pub generation: u32,
    /// In Ext2 version 0, this field is reserved. 
    /// In version >= 1, Extended attribute block (File ACL).
    /// ACL means access control list
    pub file_acl: u32,
    /// In Ext2 version 0, this field is reserved. 
    /// In version >= 1, Upper 32 bits of file size 
    /// (if feature bit set) if it's a file, 
    /// Directory ACL if it's a directory
    pub file_size: u32,
    /// I don't want to use unions at the moment so two fields
    pub directory_acl: u32,
    /// Fragment number
    pub fragment: u8,
    /// Fragment size
    pub fragment_size: u8,
    /// High 16 bits of 32 bit "Types and Permissions field" (HURD)
    pub hi_types_and_permissions: u16,
    /// High 16 bits of 32-bit User ID (LINUX, HURD)
    pub hi_uid: u16,
    /// High 16 bits of 32-bit Group ID (LINUX, HURD)
    pub hi_gid: u16,
    /// User ID of author (if == 0xFFFFFFFF, the normal User ID will be used)
    /// (HURD)
    pub uid_author: u32,
}

/// Directories are inodes which contain some number of "entries" as their contents. These entries are nothing more than a name/inode pair. For instance the inode corresponding to the root directory might have an entry with the name of "etc" and an inode value of 50. A directory inode stores these entries in a linked-list fashion in its contents blocks.
/// The root directory is Inode 2.
/// The total size of a directory entry may be longer then the length of the name would imply (The name may not span to the end of the record), and records have to be aligned to 4-byte boundaries. Directory entries are also not allowed to span multiple blocks on the file-system, so there may be empty space in-between directory entries. Empty space is however not allowed in-between directory entries, so any possible empty space will be used as part of the preceding record by increasing its record length to include the empty space. Empty space may also be equivalently marked by a separate directory entry with an inode number of zero, indicating that directory entry should be skipped.
pub struct DirEnt
{
    pub inode: u32,
    /// Total size of this entry (Including all subfields)
    pub size: u16,
    /// Name Length least-significant 8 bits
    pub name_length: u8,
    /// Type indicator (only if the feature bit for "directory 
    /// entries have file type byte" is set, else this is the 
    /// most-significant 8 bits of the Name Length)
    pub name_characters: [u8; 32],
}