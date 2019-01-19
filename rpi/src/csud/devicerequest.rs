/**
	\brief An encapsulated device request.

	A device request is a standard mechanism defined in USB2.0 manual section
	9.3 by which negotiations with devices occur. The request have a number of 
	parameters, and so are best implemented with a structure. As per usual,
	since this structure is arbitrary, we shall match Linux in the hopes of 
	achieving some compatibility.
*/
#[repr(u8)]
enum UsbDeviceRequest
{
    // USB Requests
    GetStatus: u8 = 0,
    ClearFeature: u8 = 1,
    SetFeature: u8 = 3,
    SetAddress: u8 = 5,
    GetDescriptor: u8 = 6,
    SetDescriptor: u8 = 7,
    GetConfiguration: u8 = 8,
    SetConfiguration: u8 = 9,
    GetInterface: u8 = 10,
    SetInterface: u8 = 11,
    SyncFrame: u8 = 12,
    // HID Requests
    /* Does this particulary matter the numbers? */
    // If so then I need to create a HidDeviceRequest
}

#[repr(u8)]
enum HidDeviceRequest
{
    GetReport: u8 = 1,
    GetIdle: u8 = 2,
    GetProtocol: u8 = 3,
    SetReport: u8 = 9,
    SetIdle: u8 = 10,
    SetProtocol: u8 = 11,
}

#[repr(C)]
struct UsbDeviceRequestStruct
{
    type: u8,
    request: u8, 
    /* 
     * Just cast it to either HidDeviceRequest
     * or UsbDeviceRequest
     */
    value: u16,
    index: u16,
    len: u16,
}