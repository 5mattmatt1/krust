/** 
	\brief The maximum number of children a device could have, by implication, this is 
	the maximum number of ports a hub supports. 
	
	This is theoretically 255, as 8 bits are used to transfer the port count in
	a hub descriptor. Practically, no hub has more than 10, so we instead allow 
	that many. Increasing this number will waste space, but will not have 
	adverse consequences up to 255. Decreasing this number will save a little 
	space in the HubDevice structure, at the risk of removing support for an 
	otherwise valid hub. 
*/
const MAX_CHILDREN_PER_DEVICE: u8 = 10;

/** 
	\brief The maximum number of interfaces a device configuration could have. 

	This is theoretically 255 as one byte is used to transfer the interface 
	count in a configuration descriptor. In practice this is unlikely, so we 
	allow an arbitrary 8. Increasing this number wastes (a lot) of space in 
	every device structure, but should not have other consequences up to 255.
	Decreasing this number reduces the overheads of the UsbDevice structure, at
	the cost of possibly rejecting support for an otherwise supportable device. 
*/
const MAX_INTERFACES_PER_DEVICE: u8 = 8;

/** 
	\brief The maximum number of endpoints a device could have (per interface). 
	
	This is theoretically 16, as four bits are used to transfer the endpoint 
	number in certain device requests. This is possible in practice, so we 
	allow that many. Decreasing this number reduces the space in each device
	structure considerably, while possible removing support for otherwise valid
	devices. This number should not be greater than 16.
*/
const MAX_ENDPOINTS_PER_DEVICE: u8 = 16;

const INTEFACE_CLASS_ATTACH_COUNT: u8 = 16;

/**
	\brief Status of a USB device.

	Stores the status of a USB device. Statuses as defined in 9.1 of the USB2.0 
	manual.
*/
#[repr(u8)]
enum UsbDeviceStatus
{
    attached: u8 = 0,
    powered: u8 = 1,
    default: u8 = 2,
    addressed: u8 = 3,
    configured: u8 = 4,
}

/**
	\brief Status of a USB transfer.

	Stores the status of the last transfer a USB device did.
*/
#[repr(u32)]
enum UsbTransferError
{
    no_err: u32 = 0,
    stall: u32 = 0x2, // 1 << 1,
    buf_err: u32 = 0x4, // 1 << 2,
    babble: u32 = 0x8, // 1 << 3
    no_ack: u32 = 0xF, // 1 << 4
    crc_err: u32 = 0x20, // 1 << 5
    bit_err: u32 = 0x40, // 1 << 6
    conn_err: u32 = 0x80, // 1 << 7
    ahb_err: u32 = 0x100, // 1 << 8
    not_yet_err: u32 = 0x200, // 1 << 9
    processing: u32 = 0x80000000, // 1 << 31
}

/**
	\brief Start of a device specific data field.

	The first two words of driver data in a UsbDevice. The  DeviceDriver field 
	is a code which uniquely identifies the driver that set the driver data 
	field (i.e. the lowest driver in the stack above the USB driver). The 
	DataSize is the size in bytes of the device specific data field. 
*/
struct UsbDriverDataHeader
{
    device_driver: u32,
    data_size: u32,
}

trait UsbDeviceHandler
{
    fn detached();
    fn dealloc();
    fn poll_change();
    fn child_detached();
    fn child_reset();
    fn check_connection();
}

/**
	\brief Structure to store the details of a USB device that has been 
	detectd.

	Stores the details about a connected USB device. This is not directly part
	of the USB standard, and is instead a mechanism used to control the device
	tree.
*/
#[repr(C)]
struct UsbDevice
{
    num: u32,
    speed: UsbSpeed,
    status: UsbDeviceStatus,
    cfg_index: u8, // volatile?
    port_num: u8,
    // How to align this to 4
    error: UsbTransferError,
    /* Stuff from UsbDeviceHandler */
    descriptor: UsbDeviceDescriptor,
    cfg: UsbConfigurationDescriptor,
    // Get the documentation for our heapless variants out
    // ifs: Vec<MAX_INTERFACES_PER_DEVICE: u8 = ,;
    // endpoints: Vec<MAX_INTERFACES_PER_DEVICE: u8 = 
;    parent: &UsbDevice,
    // I don't trust void pointers
    // full_cfg: void *???
    driver_data: UsbDriverDataHeader,
    last_transfer: u32
}

/**
	\brief Methods to attach a particular interface for a particular class.

	The class of the interface is the index into this array of methods. The
	array is populated by ConfigurationLoad().
*/
// How this next function decleration works is beyond me...
// fn 