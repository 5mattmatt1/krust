// Uses hcd
// Uses descriptors

/** The default timeout in ms of control transfers. */
const CONTORL_MESSAGE_TIMEOUT: u8 = 10;
/** The maximum number of devices that can be connected. */
const MAX_DEVICES: u8 = 32;

static usb_devices: Vec<&UsbDevice, MAX_DEVICES>;

fn usb_init() -> bool
{
    let mut result: bool = true;
    config_load();
    if !(result = hcd_init())
    {
        // No logging yet...
        return result;
    }
    if !(result = hcd_start())
    {
        hcd_deinit();
        return result;
    } 
    if !(result = usb_attach_root_hub())
    {
        hcd_stop();
        hcd_deinit();
        return result;
    }
}

/* 
 * These are all functions that use UsbDevive
 * as the first argument so it made sense to use
 * them in an impl block
 */
impl UsbDevice
{
    // Why is buf a void *
    /**
        \brief Gets the descriptor for a given device.

        Gets the descriptor for a given device, using the index and language id 
        specified. The returned value is not longer than length.
    */
    fn get_descriptor(&self, type: DescriptorType,
        index: u8, lang_id: u16, buf: str, 
        len: u32, min_len: u32, recpient: u8) -> bool
    {
        let mut result: bool = true;
        if !(result = self.control_message(
            UsbPipeAddress::new(
                // Probably an enum...
                Control,
                self.speed,
                0,
                self.num,
                // Probably an enum...
                In,
                self.descriptor.max_packet_size0
            ),
            buf,
            len,
            UsbDeviceRequest::new(
                // Probably an enum...
                GetDescriptor,
                0x80 | recpient,
                type << 8 | index,
                lang_id,
                len,
            ),
            CONTORL_MESSAGE_TIMEOUT
        ))
        {
            return result;
        }

        if (self.last_transfer < min_len)
        {
            return false; // ErrorDevice
        }
        return result;
    }

    // Why is buf a void *
    /**
        \brief Sends a control message synchronously to a given device.

        Sends a contorl message synchronously to a given device, then waits for 
        completion. If the timeout is reached returns ErrorTimeout. This puts
        device into an inconsistent state, so best not to use it until processing
        is unset.
    */
    fn control_message(&self, 
        pipe: UsbPipeAddress, buf: str, buf_len: u32
        request: UsbDeviceRequest, timeout: u32) -> bool
    {
        let mut result: bool = true;
        if (buf as u32 & 0x3) != 0
        {
            // Just logging here
        }
        if !(result = hcd_submit_control_msg(self, pipe, buf, buf_len, request))
        {
            // Log
            return result;
        }

        while timeout-- > 0 && self.error & processing
        {
            micro_delay(1000);
        }

        if self.error & processing
        {
            return false; // ErrorTimeout
        }else if self.error & ~processing
        {
            if Some(self.parent)
            {
                if !(result = device.parent.check_connection(self.parent, self))
                {
                    return false; // ErrorDisconnected
                }
            }
            result = false; // ErrorDevice
        }
        return result;
    }

    // Need to figure out how I'll handle allocation...
    /**
        \brief Allocates memory to a new device.

        Sets the value in the parameter device to the address of a new device 
        allocated on the heap, which then has appropriate default values.
    */
    fn alloc_device() -> Self
    {
        // I need to figure out the allocation mechanism...

        for number in 0...MAX_DEVICES
        {
            if !Some(usb_devices[number])
            {
                usb_devices[number] = dev;
                dev.num = number + 1;
                break;
            }
        }
        
        dev.status = Attached;
        dev.err = NoError;
        dev.port_num = 0;
        dev.parent = None;
        dev.driver_data = None;
        dev.full_cfg = None;
        dev.cfg_index = 0xFF;
        // dev.dealloc
        // dev.detached
        // dev.check_connection
        // dev.check_for_change
        // dev.child_detached
        // dev.child_reset
    }

    /*
        \brief Deallocates the memory and resources of a USB device.

        Recursively deallocates the device and all children. Deallocates any class
        specific data, as well as the device structure itself and releases the 
        device number.
    */
    fn dealloc_device(&self) -> bool
    {
        self.detached();
        self.dealloc();
        if Some(self.parent)
        {
            self.parent.child_detached(self.parent, self);
        }

        if self.status == addressed || self.status == configured
        {
            if dev.num > 0 && dev.num < MAX_DEVICES && 
                usb_devices[dev.num - 1] == dev
            {
                usb_devices[dev.num - 1] = None;
            }
            // Could maybe use traits to get around void pointers?
            memory_dealloc(dev.full_cfg);
        }

        memory_dealloc(dev);
    }

    /**
        \brief Recursively enumerates a new device.

        Recursively enumerates a new device that has been allocated. This assigns
        an address, determines what the device is, and, if it is a hub, will 
        configure the device recursively look for new devices. If not, it will 
        configure the device with the default configuration.
    */
    fn attach_device(&self) -> bool
    {
        
    }

    // Need heapless string here...
    /**
        \brief Returns a description for a device.

        Returns a description for a device. This is not read from the device, this 
        is just generated given by the driver.
    */
    fn get_description(&self) -> String<>
    {

    }
    // I don't think that UsbGetString is very important...

    fn attach_devie(&mut self) -> bool
    {
        let mut result: bool = true;
        let address: u8;
        let buffer: String<>;

        address = self.num;
        self.num = 0;
        if !(result = self.read_device_descriptor())
        {
            dev.num = address;
            return result;
        }
        self.status = DefaultStatus;

        if Some(self.parent)
        {
            if !(result = self.parent.child_reset(dev))
            {
                self.num = address;
                return result;
            }
        }

        if !(result = self.set_address(address))
        {
            self.num = address;
            return result;
        }
        self.num = address;

        if !(result = self.read_device_descriptor())
        {
            return result;
        }
        // Need a heapless string buffer of length 256
        // Note useful until logging is implemented
        /*
        let desc_prod = self.descriptor.product;
        let desc_manu = self.descriptor.manufacturer;
        let desc_sern = self.descriptor.serial_number;
        if (desc_prod != 0)
        {
            // When wouldn't it be NULL though...
            // Could reduce complexity by killing off
            //  this if statement
            // if !Some(buf)
            // {
            // If we get the value from read_string
            // Then we avoid double allocating
            // buf = String<U256>::new();
            // }
            // if Some(buf)
            // {
            buf = self.read_string(desc_prod);
            // }
        }

        if (desc_manu != 0)
        {
            buf = self.read_string(desc_manu);
        }

        if (desc_sern != 0)
        {
            buf = self.read_string(desc_sn);
        }
        */
        // Figure out how to free the heapless string
        // We only support devices with 1 configuration for now.
        if !(result = self.configure(0))
        {
            return true;
        }
        // What the fuck is the value of Interface.class
        let if_class = self.ifs[0].class;
        if (if_class < INTERFACE_CLASS_ATTACH_COUNT &&
            Some(interface_class_attach[if_class))
        {
            // result =  
            // Whats with the calling of the object???
        }

        return result;
    }

    fn configure(&mut self, config: u8) -> bool
    {
        // Not loving fullDescriptor being void *
        let mut result: bool = true;
        let lastIF: u32;
        let lastEndPoint: u32;
        let isAlternate: bool;
        // Some parts of get_descriptor
        // Can probably be simplified using
        // rust's constructs
        if (self.status != Addressed)
        {
            return false; //ErrorDevice
        }

        if !(result = self.get_descriptor(Configuration, config, 0, self.config, 0))
        {
            return result;
        }
        // Better to use rust's tuple return
        self.config_index = config;
        config = dev.cfg.cfg_val;
        // header = fullDesc;
        lastIF = MAX_INTERFACES_PER_DEVICE;
        lastEndPoint = MAX_ENDPOINTER_PER_DEVICE;
        isAlternate = false;
        // Is this absolutely neccessary
        // Can I not do a safer version of this?
        // I suppose this isn't entirely unsafe
        // But it is not good practice
        unsafe
        {
        
        let full_desc_addr: u32 = &full_desc as *const UsbHeader as usize;
        let header_addr: u32 = full_desc_addr;
        let header = *(header_addr as *const UsbHeader);
        // I don't trust all this type casting...
        /*
        // UsbDescriptorHeader
        // UsbInterfaceDescriptor
        // Compare these two to figure out
        // How they can both be pulled from the same
        // Memory
        while head_addr - full_desc_addr < self.cfg.total_len
        {
            match header.desc_type
            {
                Interface =>
                {
                    
                }
                Endpoint =>
                {

                }
                _ =>
                {

                }
            }
            header_addr += header.desc_len;
        }

        }
        */
    }

    fn get_desc(&self) -> String<>
    {
        /*
        // Wouldn't hurt to see if I can pull
        // Any of this shit out or simplify the
        // names.
        */
        // It would seriously not hurt to
        // Get some fucking hash maps going here...
        match self.status
        {
            Attached => return String::from(
                "New Device (Not Ready"),
            Powered => return String::from(
                "Unknown Device (Not ready"),
            
        }
        let desctor = self.descriptor;
        match desctor.class
        {
            DeviceClassHub =>
            {
                match desctor.usb_version
                {
                    0x210 => return String::from("USB 2.1 Hub"),
                    0x200 => return String::from("USB 2.0 Hub"),
                    0x110 => return String::from("USB 1.1 Hub"),
                    0x100 => return String::from("USB 1.0 Hub"),
                    _ => return String::from("USB Hub"),
                };
            },
            DeviceClassVendorSpecific =>
            {
                // Very important for my network stack..
                // Definitely where a hash map could be used
                // for modular loading
                if desctor.vendor_id == 0x424 &&
                    desctor.product_id == 0xEC00
                {
                    return String::from("SMSC LAN9512");
                }
            }
            DeviceClassInInterface =>
            {
                if self.status == Configured
                {
                    match self.ifs[0].class
                    {
                        InterfaceClassAudio => return String::from("USB Audio Device"),
                        InterfaceClassCommunications => return String::from("USB CDC Device"),
                        InterfaceClassHid =>
                        {
                            match self.ifs[0].proto
                            {
                                1 => return String::from("USB Keyboard"),
                                2 => return String::from("USB Mouse"),
                                _ => return String::from("USB HID"),
                            }
                        }
                        InterfaceClassPhysical => return String::from("USB Physical Device"),
                        InterfaceClassImage => return String::from("USB Imaging Device"),
                        InterfaceClassPrinter => return String::from("USB Printer"),
                        InterfaceClassMassStorage => return String::from("USB Mass Storage Device"),
                        InterfaceClassHub => 
                        {
                            match desctor.usb_version
                            {
                                0x210 => return String::from("USB 2.1 Hub"),
                                0x200 => return String::from("USB 2.0 Hub"),
                                0x110 => return String::from("USB 1.1 Hub"),
                                0x100 => return String::from("USB 1.0 Hub"),
                                _ => return String::from("USB Hub"),
                            };
                        }
                        InterfaceClassCdcData => return String::from("USB CDC-Data Device"),
                        InterfaceClassSmartCard => return String::from("USB Smart Card"),
                        InterfaceClassContentSecurity => return String::from("USB Content Security Device"),
                        InterfaceClassVideo => return String::from("USB Video Device"),
                        InterfaceClassPersonalHealthCare => return String::from("USB Healthcare Device"),
                        InterfaceClassAudioVideo => return String::from("USB AV Device"),
                        InterfaceClassDiagnosticDevice => return String::from("USB Diagnostic Device"),
                        InterfaceClassWirelessController => return String::from("USB Wireless Controller"),
                        InterfaceClassMiscellaneous => return String::from("USB Miscellaneous Device"),
                        InterfaceClassVendorSpecific => return String::from("Vendor Specific"),
                        _ => return String::from("Generic Device"),
                    },
                    DeviceClassVendorSpecific => return String::from("Vendor Specific"),
                    _ => return String::from("Unconfigured Device"),
                },
                _ => return String::from("Generic Device"),
            }
        }
    }
}

fn usb_attach_root_hub()
{
    let mut result: bool = true;
    let root_hub: UsbDevice = UsbDevice::new();
    if Some(usb_devices[0])
    {
        usb_devices.dealloc();
    }

    root_hub.status = Powered;
    return root_hub.attach_device();
}

// Honestly not that useful and could use some
// renaming
fn check_for_change()
{
    let root_hub: &UsbDevice = usb_devices[0];
    if Some(root_hub)
    {
        root_hub.check_for_change();
    }
}

/**
    \brief Returns a pointer to the root hub device.

    On a Universal Serial Bus, there exists a root hub. This if often a virtual
    device, and typically represents a one port hub, which is the physical 
    universal serial bus for this computer. It is always address 1. It is 
    present to allow uniform software manipulation of the universal serial bus 
    itself.
*/
fn usb_get_root_hub() -> &UsbDevice
{
    return usb_devices[0];
}