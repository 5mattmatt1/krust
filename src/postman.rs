// Based off tutorial from:
// https://www.cl.cam.ac.uk/projects/raspberrypi/tutorials/os/screen01.html

// Mailbox addresses

const READ_ADDR: u32 = 0x2000B880;
const POLL_ADDR: u32 = 0x2000B890;
const SENDER_ADDR: u32 = 0x2000B894;
const STATUS_ADDR: u32 = 0x2000B898;
const CONFIG_ADDR: u32 = 0x2000B89C;
const WRITE_ADDR: u32 = 0x2000B8A0;

const HIGH_COLOR_BIT_DEPTH: u32 = 16;
const TRUE_COLOR_BIT_DEPTH: u32 = 24;
const RGBA32_BIT_DEPTH: u32 = 32;

struct FrameBufferInfo
{
    phy_width: u32,
    phy_height: u32,
    virt_width: u32,
    virt_height: u32,
    gpu_pitch: u32,
    bit_depth: u32, /* Could benefit from an enum */
    x: u32,
    y: u32,
    gpu_ptr: u32,
    gpu_size: u32,
}

fn mailbox_write()
{

}