/*
 * Will need to do a stat on FAT32...
 */

#[repr(C)]
struct Stat
{
    /* st_dev */
    st_ino: usize,
    st_mode: usize,
    nlink: usize,
    uid: usize,
    gid: usize,
    rdev: usize,
    size: usize,
    blksize: usize,
    blocks: usize,
}

fn stat(pathname: &str, stat: &mut Stat) -> i32
{
    /* Syscall that I need to implement */
}

// read
// write
// 