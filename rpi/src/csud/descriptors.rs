/**
	\brief The descriptor type field from the header of USB descriptors.

	The descriptor type in the header of all USB descriptor sturctures defined
	in the USB 2.0 manual in 9.6.
*/
#[repr(u8)]
enum DescriptorType
{
    Device: u8 = 1,
    Cfg: u8 = 2,
    Str: u8 = 3,
    IF: u8 = 4,
    Endpoint: u8 = 5, // EP?
    DeviceQualifier: u8 = 6,
    OtherSpeedCfg: u8 = 7,
    IFPwr: u8 = 8,
    Hid: u8 = 33,
    HidReport: u8 = 34,
    HidPhy: u8 = 35,
    Hub: u8 = 41,
}

/**
	\brief The header of USB descriptor information.

	The header of all USB descriptor sturctures defined in the USB 2.0 manual 
	in 9.6.
*/
struct UsbDescriptorHeader
{
    desc_len: u8,
    desc_type: DescriptorType,
}
// __attribute__ ((__packed__)) // ???

/**
	\brief The device descriptor information.

	The device descriptor sturcture defined in the USB 2.0 manual in 9.6.1.
*/

#[repr(u8)]
enum DeviceClass
{
    DevClsInIF: u8 = 0x00, // DeviceClassInInterface
    DevClsComm: u8 = 0x02, // DeviceClassCommunications
    DevClsHub: u8 = 0x09, // DeviceClassHub
    DevClsDiag: u8 = 0xDC, // DeviceClassDiagnostic
    DevClsMisc: u8 = 0xEF, // DeviceClassMiscellaneous
    DevClsVendorSpec: u8 = 0xFF, // DeviceClassVendorSpecific
}

#[repr(C)]
struct UsbDeviceDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    usb_version: u16,
    cls: DeviceClass,
    subclass: u8,
    proto: u8,
    max_pkt_size0: u8,
    vendor_id: u16,
    prod_id: u16,
    version: u16,
    manu: u8,
    prod: u8,
    sn: u8,
    cfg_count: u8,
}

/**
	\brief The device qualifier descriptor information.

	The device descriptor qualifier sturcture defined in the USB 2.0 manual in 
	9.6.2.
*/
#[repr(C)]
struct UsbDeviceQualifierDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    usb_version: u16,
    cls: DeviceClass,
    subclass: u8,
    proto: u8,
    max_pkt_size0: u8,
    cfg_count: u8,
    _reserved9: u8,
}

/**
	\brief The configuration descriptor information.

	The configuration descriptor structure defined in the USB2.0 manual section
	9.6.3.
*/
#[repr(C)]
struct UsbConfigurationDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    total_len: u16,
    if_count: u8,
    cfg_val: u8,
    stri: u8,
    // What type of type is unsigned???
    _reserved0_4: u32,
    remote_wakup: bool,
    self_powered: bool,
    _reserved7: u32,
    max_pow: u8,
}

/**
	\brief The other speed configuration descriptor.

	The other speed configuration descriptor defined in the USB2.0 manual section 
	9.6.4.
*/
struct UsbOtherSpeedConfigurationDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    total_len: u16,
    if_count: u8,
    cfg_val: u8,
    stri: u8,
    /* Attributes */
    _reserved0_4: u32,
    remote_wakup: bool,
    self_powered: bool,
    _reserved7: u8,
    /* End attributes */
    max_pow: u8,
}

/* I half want to get rid of the IFCls's too... */
enum InterfaceClass {
    IFClsReserved = 0x0,
    IFClsAudio = 0x1,
    IFClsCommunications = 0x2,
    IFClsHid = 0x3,
    IFClsPhysical = 0x5,
    IFClsImage = 0x6,
    IFClsPrinter = 0x7,
    IFClsMassStorage = 0x8,
    IFClsHub = 0x9,
    IFClsCdcData = 0xa,
    IFClsSmartCard = 0xb,
    IFClsContentSecurity = 0xd,
    IFClsVideo = 0xe,
    IFClsPersonalHealthcare = 0xf,
    IFClsAudioVideo = 0x10,
    IFClsDiagnosticDevice = 0xdc,
    IFClsWirelessController = 0xe0,
    IFClsMiscellaneous = 0xef,
    IFClsApplicationSpecific = 0xfe,
    IFClsVendorSpecific = 0xff,
}

/**
	\brief The interface descriptor information.

	The interface descriptor structure defined in the USB2.0 manual section 
	9.6.5.
*/
#[repr(C)]
struct UsbInterfaceDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    num: u8,
    alt_setting: u8,
    endpoint_count: u8,
    cls: InterfaceClass,
    subclass: u8,
    proto: u8,
    stri: u8,
}

/* 
 * Getting rid of some of some of the 
 * anonymous structs and enums in favor for
 * top level declarations
 */
#[repr(u8)]
enum Synchronisation
{
    NoSync: u8 = 0,
    Async: u8 = 1,
    Adapt: u8 = 2,
    Syncron: u8 = 3,
}

#[repr(u8)]
enum Usage
{
    Data: u8 = 0,
    Feedback: u8 = 1,
    ImplicitFeedbackData: u8 = 2,
}

#[repr(u8)]
enum Transaction
{
    None: u8 = 0,
    Extra1: u8 = 1,
    Extra2: u8 = 2,
}

#[repr(C)]
struct EndpointAddress
{
    num: u32,
    _reserved4_6: u8,
    dir: UsbDirection,
}

#[repr(C)]
struct UsbPacket
{
    max_size: u32,
    trans: Transaction,
    _reserved13_15: u32,
}

/**
	\brief The endpoint descriptor information.

	The endpoint descriptor structure defined in the USB2.0 manual section 
	9.6.6.
*/
struct UsbEndpointDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    ep_addr: EndpointAddress,
    /* Attributes */
    // TODO:
    // Decide if I want to make a
    // UsbEndpointAttributes struct
    trans_type: UsbTransfer,
    sync: Synchronisation,
    usage: Usage,
    _reserved6_7: u8,
    /* End attributes */
    pkt: UsbPacket,
    int: u8,
}

/**
	\brief The string descriptor information.

	The string descriptor structure defined in the USB2.0 manual section 
	9.6.7.
*/
struct UsbStringDescriptor
{
    desc_len: u8,
    desc_type: DescriptorType,
    data: u16[],
}