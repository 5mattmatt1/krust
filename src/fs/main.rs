/// Some details about the module.
/// 
/// More detail at: [The BTRFS Wiki](https://btrfs.wiki.kernel.org/index.php/Main_Page)
pub mod btrfs;

/// The Second Extended Filesystem (ext2fs) is a rewrite of the original 
/// Extended Filesystem and as such, is also based around the concept 
/// of "inodes." Ext2 served as the de facto filesystem of Linux for
/// nearly a decade from the early 1990s to the early 2000s when it 
/// was superseded by the journaling file systems ext3 and ReiserFS. 
/// It has native support for UNIX ownership / access rights, symbolic- and hard-links, 
/// and other properties that are common among UNIX-like operating systems. Organizationally, 
/// it divides disk space up into groups called "block groups." 
/// Having these groups results in distribution of data across the disk 
/// which helps to minimize head movement as well as the impact of fragmentation. 
/// Further, some (if not all) groups are required to contain backups of important data 
/// that can be used to rebuild the file system in the event of disaster.
///
/// More detail at: [EXT2 OSDev](https://wiki.osdev.org/Ext2)
pub mod ext2;

fn main() {
    println!("Hello, world!");
}
