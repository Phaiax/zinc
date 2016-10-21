// Zinc, the bare metal stack for rust.
// Copyright 2016 Daniel Seemer 'phaiax' <phaiax-zinc@invisibletower.de>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Write;
use std::cmp::max;

pub enum DescriptorTypes {
    Device,
    Configuration,
    String,
    Interface,
    Endpoint,
    DeviceQualifier,
    OtherSpeedConfiguration,
    InterfacePower,
    OnTheGo,
    Cdc,
}

impl DescriptorTypes {
    pub fn id(&self) -> u8 {
        match self {
            &DescriptorTypes::Device => 1,
            &DescriptorTypes::Configuration => 2,
            &DescriptorTypes::String => 3,
            &DescriptorTypes::Interface => 4,
            &DescriptorTypes::Endpoint => 5,
            &DescriptorTypes::DeviceQualifier => 6,
            &DescriptorTypes::OtherSpeedConfiguration => 7,
            &DescriptorTypes::InterfacePower => 8,
            &DescriptorTypes::OnTheGo => 9,
            &DescriptorTypes::Cdc => 0x24,
        }
    }
}

pub enum CDCDescriptorSubTypes {
    CDCHeader,
    CallManagement,
    AbstactControl,
    UnionFunctional,
}

impl CDCDescriptorSubTypes {
    pub fn id(&self) -> u8 {
        match self {
            &CDCDescriptorSubTypes::CDCHeader => 0,
            &CDCDescriptorSubTypes::CallManagement => 1,
            &CDCDescriptorSubTypes::AbstactControl => 2,
            &CDCDescriptorSubTypes::UnionFunctional => 6,
        }
    }
}

pub enum BcdUsb {
    Usb10,
    Usb11,
    Usb20
}

impl BcdUsb {
    pub fn id(&self) -> u16 {
        match self {
            &BcdUsb::Usb10 => 0x0100,
            &BcdUsb::Usb11 => 0x0110,
            &BcdUsb::Usb20 => 0x0200,
        }
    }
}

pub struct Bcd(pub u16);

impl Bcd {
    pub fn validate(&self) {
        // TODO
        // Panic when invalid
    }
}


pub enum DeviceClass {
    EachInterfaceDefinesOwnClass,
    VendorDefinedClass,
    ClassCode(u8),
}

impl DeviceClass {
    pub fn id(&self) -> u8 {
        match self {
            &DeviceClass::EachInterfaceDefinesOwnClass => 0x00,
            &DeviceClass::VendorDefinedClass => 0xFF,
            &DeviceClass::ClassCode(cc) => cc,
        }
    }
}

pub enum MaxPacketSizeEP0 {
    Bytes8,
    Bytes16,
    Bytes32,
    Bytes64,
}


impl MaxPacketSizeEP0 {
    pub fn id(&self) -> u8 {
        match self {
            &MaxPacketSizeEP0::Bytes8 => 8,
            &MaxPacketSizeEP0::Bytes16 => 16,
            &MaxPacketSizeEP0::Bytes32 => 32,
            &MaxPacketSizeEP0::Bytes64 => 64,
        }
    }
}

pub struct StringId(u8, String);

impl StringId {
    pub fn new(s : &str) -> StringId {
        StringId(0, s.into())
    }
}

pub struct ConfAttributes {
    pub self_powered : bool,
    pub remote_wakeup : bool,
}

impl ConfAttributes {
    pub fn id(&self) -> u8 {
        let mut b = 0b1000_0000;
        if self.self_powered {
            b |= 0b0100_0000;
        }
        if self.remote_wakeup {
            b |= 0b0010_0000;
        }
        b
    }
}

pub struct MaxCurrent {
    pub milli_amps : u32,
}

impl MaxCurrent {
    pub fn id(&self) -> u8 {
        if self.milli_amps % 2 != 0 {
            panic!("Usb current must be given as multiple of two.");
        }
        if self.milli_amps > 500 {
            panic!("Usb current must not be greater 500 mA.");
        }
        (self.milli_amps / 2) as u8
    }
}

/// The number is the enpoint number
pub enum EndpointAddress {
    In(u8),
    Out(u8),
}

impl EndpointAddress {
    pub fn id(&self) -> u8 {
        match *self {
            EndpointAddress::Out(nr) => {
                if nr >= 16 {
                    panic!("Maximum usb endpoint number is 15");
                }
                nr
            },
            EndpointAddress::In(nr) => {
                if nr >= 16 {
                    panic!("Maximum usb endpoint number is 15");
                }
                0b1000_0000 | nr
            }
        }
    }
    pub fn addr(&self) -> u8 {
        match *self {
            EndpointAddress::Out(nr) => nr,
            EndpointAddress::In(nr) => nr,
        }
    }
}

pub type EndpointAttributes = TransferType;

pub enum TransferType {
    Control,
    Isochronous(SyncType, UsageType),
    Bulk,
    Interrupt,
}

pub enum SyncType {
    NoSync,
    Async,
    Adaptive,
    Sync
}

pub enum UsageType {
    Data,
    Feedback,
    ImplicitFeedbackData,
}


impl TransferType {
    pub fn id(&self) -> u8 {
        match self {
            &TransferType::Control => 0,
            &TransferType::Isochronous(ref sync, ref usage) => {
                1 | (sync.id() << 2) | (usage.id() << 4)
            },
            &TransferType::Bulk => 2,
            &TransferType::Interrupt => 3,
        }
    }
    pub fn is_isochronus(&self) -> bool {
        match self {
            &TransferType::Isochronous(_,_) => true,
            _ => false,
        }
    }
}

impl SyncType {
    pub fn id(&self) -> u8 {
        match self {
            &SyncType::NoSync => 0,
            &SyncType::Async => 1,
            &SyncType::Adaptive => 2,
            &SyncType::Sync => 3,
        }
    }
}

impl UsageType {
    pub fn id(&self) -> u8 {
        match self {
            &UsageType::Data => 0,
            &UsageType::Feedback => 1,
            &UsageType::ImplicitFeedbackData => 2,
        }
    }
}


#[allow(non_snake_case)]
pub struct DeviceDescriptor {
    pub bcdUSB : BcdUsb,
    pub bDeviceClass : DeviceClass,
    pub bDeviceSubClass : u8,
    pub bDeviceProtocol : u8,
    pub bMaxPacketSize0 : MaxPacketSizeEP0,
    pub idVendor : u16,
    pub idProduct : u16,
    pub bcdDevice : Bcd,
    pub iManufacturer : StringId,
    pub iProduct : StringId,
    pub iSerialNumber : StringId,
    pub configurations : Vec<ConfigurationDescriptor>,
}

impl DeviceDescriptor {
    pub fn source(&self) -> String {
        format!(r#"
    pub const DEVICEDESCRIPTOR: &'static [u8] = &[
        0x{bLength:x},      // bLength
        0x{bDescriptorType:x},      // bDescriptorType
        0x{bcdUSB_LSB:x}, 0x{bcdUSB_MSB:x},// bcdUSB
        0x{bDeviceClass:x},      // bDeviceClass
        0x{bDeviceSubClass:x},      // bDeviceSubClass
        0x{bDeviceProtocol:x},      // bDeviceProtocol
        0x{bMaxPacketSize0:x},      // bMaxPacketSize0
        0x{idVendor_LSB:x}, 0x{idVendor_MSB:x},// idVendor
        0x{idProduct_LSB:x}, 0x{idProduct_MSB:x},// idProduct
        0x{bcdDevice_LSB:x}, 0x{bcdDevice_MSB:x},// bcdDevice
        0x{iManufacturer:x},      // iManufacturer
        0x{iProduct:x},      // iProduct
        0x{iSerialNumber:x},      // iSerialNumber
        0x{bNumConfigurations:x},      // bNumConfigurations
    ];
             "#,
            bLength = 18,
            bDescriptorType = DescriptorTypes::Device.id(),
            bcdUSB_MSB = msb(self.bcdUSB.id()),
            bcdUSB_LSB = lsb(self.bcdUSB.id()),
            bDeviceClass = self.bDeviceClass.id(),
            bDeviceSubClass = self.bDeviceSubClass,
            bDeviceProtocol = self.bDeviceProtocol,
            bMaxPacketSize0 = self.bMaxPacketSize0.id(),
            idVendor_MSB = msb(self.idVendor),
            idVendor_LSB = lsb(self.idVendor),
            idProduct_MSB = msb(self.idProduct),
            idProduct_LSB = lsb(self.idProduct),
            bcdDevice_MSB = msb(self.bcdDevice.0),
            bcdDevice_LSB = lsb(self.bcdDevice.0),
            iManufacturer = self.iManufacturer.0,
            iProduct = self.iProduct.0,
            iSerialNumber = self.iSerialNumber.0,
            bNumConfigurations = self.configurations.len() )
    }
}

#[allow(non_snake_case)]
pub struct ConfigurationDescriptor {
    /// will be calculated automatically
    pub wTotalLength : u16,
    pub interfaces : Vec<InterfaceDescriptor>,
    pub bConfigurationValue : u8,
    pub iConfiguration : StringId,
    pub bmAttributes : ConfAttributes,
    pub bMaxPower : MaxCurrent,
}

impl ConfigurationDescriptor {
    pub fn len(&self) -> u8 { 9 }
    pub fn source(&self) -> String {
        format!(r#"
        // CONFIGURATION DESCRIPTOR
            0x{bLength:x},      // bLength
            0x{bDescriptorType:x},      // bDescriptorType
            0x{wTotalLength_LSB:x}, 0x{wTotalLength_MSB:x},// wTotalLength
            0x{bNumInterfaces:x},      // bNumInterfaces
            0x{bConfigurationValue:x},      // bConfigurationValue
            0x{iConfiguration:x},      // iConfiguration
            0x{bmAttributes:x},      // bmAttributes
            0x{bMaxPower:x},      // bMaxPower
            "#,
            bLength = self.len(),
            bDescriptorType = DescriptorTypes::Configuration.id(),
            wTotalLength_LSB = lsb(self.wTotalLength),
            wTotalLength_MSB = msb(self.wTotalLength),
            bNumInterfaces = self.interfaces.len(),
            bConfigurationValue = self.bConfigurationValue,
            iConfiguration = self.iConfiguration.0,
            bmAttributes = self.bmAttributes.id(),
            bMaxPower = self.bMaxPower.id())
    }
}

#[allow(non_snake_case)]
pub struct InterfaceDescriptor {
    pub bInterfaceNumber : u8,
    pub bAlternateSetting : u8,
    pub endpoints : Vec<EndpointDescriptor>,
    pub bInterfaceClass : u8,
    pub bInterfaceSubClass : u8,
    pub bInterfaceProtocol : u8,
    pub iInterface : StringId,
    pub cdcs : Vec<CDCDescriptor>,
}

impl InterfaceDescriptor {
    pub fn len(&self) -> u8 { 9 }
    pub fn source(&self) -> String {
        format!(r#"
        // INTERFACE
            0x{bLength:x},      // bLength
            0x{bDescriptorType:x},      // bDescriptorType
            0x{bInterfaceNumber:x},      // bInterfaceNumber
            0x{bAlternateSetting:x},      // bAlternateSetting
            0x{bNumEndpoints:x},      // bNumEndpoints
            0x{bInterfaceClass:x},      // bInterfaceClass
            0x{bInterfaceSubClass:x},      // bInterfaceSubClass
            0x{bInterfaceProtocol:x},      // bInterfaceProtocol
            0x{iInterface:x},      // iInterface
            "#,
            bLength = self.len(),
            bDescriptorType = DescriptorTypes::Interface.id(),
            bInterfaceNumber = self.bInterfaceNumber,
            bAlternateSetting = self.bAlternateSetting,
            // TODO Check, bNumEndpoints must not include control endpoint 0
            bNumEndpoints = self.endpoints.len(),
            bInterfaceClass = self.bInterfaceClass,
            bInterfaceSubClass = self.bInterfaceSubClass,
            bInterfaceProtocol = self.bInterfaceProtocol,
            iInterface = self.iInterface.0)
    }
}


#[allow(non_snake_case)]
pub struct CDCDescriptor {
    pub bDescriptorSubType : CDCDescriptorSubTypes,
    pub bytes : Vec<u8>,
}

impl CDCDescriptor {
    pub fn len(&self) -> u8 { 3 + self.bytes.len() as u8 }
    pub fn source(&self) -> String {
        let mut s = format!(r#"
        // CDC
            0x{bLength:x},      // bLength
            0x{bDescriptorType:x},      // bDescriptorType
            0x{bDescriptorSubType:x},      // bDescriptorSubType
            "#,
            bLength = self.len(),
            bDescriptorType = DescriptorTypes::Cdc.id(),
            bDescriptorSubType = self.bDescriptorSubType.id());
        for b in self.bytes.iter() {
            s.push_str(&format!("0x{:x}, ", b));
        }
        s
    }
}

#[allow(non_snake_case)]
pub struct EndpointDescriptor {
    pub bEndpointAddress : EndpointAddress,
    pub bmAttributes : EndpointAttributes,
    pub wMaxPacketSize : u16,
    pub bInterval : u8,
}

impl EndpointDescriptor {
    pub fn len(&self) -> u8 { 7 }
    pub fn source(&self) -> String {
        format!(r#"
        // ENDPOINT
            0x{bLength:x},      // bLength
            0x{bDescriptorType:x},      // bDescriptorType
            0x{bEndpointAddress:x},      // bEndpointAddress
            0x{bmAttributes:x},      // bmAttributes
            0x{wMaxPacketSize_LSB:x}, 0x{wMaxPacketSize_MSB:x},// wMaxPacketSize
            0x{bInterval:x},      // bInterval
            "#,
            bLength = self.len(),
            bDescriptorType = DescriptorTypes::Endpoint.id(),
            bEndpointAddress = self.bEndpointAddress.id(),
            bmAttributes = self.bmAttributes.id(),
            wMaxPacketSize_MSB = msb(self.wMaxPacketSize),
            wMaxPacketSize_LSB = lsb(self.wMaxPacketSize),
            bInterval = self.bInterval)
    }
}

/// Support one language only
#[allow(non_snake_case)]
pub struct StringDescriptorZero {
    wLANGID : Vec<u16>,
}

impl StringDescriptorZero {
    pub fn default() -> StringDescriptorZero {
        StringDescriptorZero {
            wLANGID : vec![0x0409]
        }
    }
    pub fn len(&self) -> u8 { (2 + self.wLANGID.len() * 2) as u8 }
    pub fn source(&self) -> String {
        let mut s = format!(r#"
    const STRINGZERODESCRIPTOR: &'static [u8] = &[
        0x{bLength:x},      // bLength
        0x{bDescriptorType:x},      // bDescriptorType
        "#,
            bLength = self.len(),
            bDescriptorType = DescriptorTypes::String.id());
        for langid in self.wLANGID.iter() {
            s.push_str(&format!("0x{:x}, 0x{:x}\n", lsb(*langid), msb(*langid)));
        }
        s.push_str("    ];");
        s
    }
}

#[allow(non_snake_case)]
struct StringDescriptor {
    bString : String,
    id : u8,
}

impl StringDescriptor {
    fn new(s : &str, id : u8) -> StringDescriptor {
        if s.as_bytes().len() > 253 {
            panic!("Usb descriptor string to large. Max 253 bytes.")
        }
        StringDescriptor {
            bString : s.into(),
            id : id,
        }
    }
    pub fn len(&self) -> u8 { (2 + self.bString.as_bytes().len()) as u8 }
    pub fn source(&self) -> String {
        let mut s = format!(r#"
    pub const STRING_{}_DESCRIPTOR: &'static [u8] = &[
        0x{bLength:x},      // bLength
        0x{bDescriptorType:x},      // bDescriptorType
        // {bString}
        "#,
            self.id,
            bLength = self.len(),
            bDescriptorType = DescriptorTypes::String.id(),
            bString = self.bString);
        for byte in self.bString.as_bytes().iter() {
            s.push_str(&format!("0x{:x}, ", byte));
        }
        s.push_str("\n    ];");
        s
    }
}

fn lsb(i : u16) -> u8 {
    i as u8
}

fn msb(i : u16) -> u8 {
    (i >> 8) as u8
}

/// The hex values are the value the USBx_ENDPTn register must be set to to configure that endpoint
/// correctly. See p994 of manual. Bits [0 0 0 enableControlTransfers enableRx enableTx isStalled !isochronous]
#[derive(Default, Copy, Clone)]
struct Endpointconfig(u8);

impl Endpointconfig {
    fn set_rx(&mut self) { self.0 |= 0b0001_1000 }
    fn rx(&mut self) -> bool { self.0 & 0b0000_1000 != 0 }
    fn set_tx(&mut self) { self.0 |= 0b0001_0100 }
    fn tx(&mut self) -> bool { self.0 & 0b0000_0100 != 0 }
    fn set_notisochronous(&mut self) { self.0 |= 0b0000_0001 }
}

pub struct DescriptorTree {
    device : DeviceDescriptor,
    /// Id of the fallback string descriptor
    miss : u8,
    strings : Vec<StringDescriptor>,
    string0 : StringDescriptorZero,
}

impl DescriptorTree {
    pub fn new(device : DeviceDescriptor) -> DescriptorTree {
        DescriptorTree {
            device : device,
            strings : vec![],
            string0 : StringDescriptorZero::default(),
            miss : 255,
        }
    }

    pub fn source(&mut self) -> Vec<String> {
        if self.device.configurations.len() > 1 {
            panic!("Usb devicetree: Maximum of one configuration descriptor supported.");
        }

        self.collect_strings_calc_len();

        let mut source = vec![];

        source.push(self.device.source());

        if let Some(configdescr) = self.device.configurations.iter().next() {
            source.push("\n    pub const CONFIGDESCRIPTORTREE: &'static [u8] = &[".into());
            source.push(configdescr.source());
            for interfacedescr in configdescr.interfaces.iter() {
                source.push(interfacedescr.source());
                for cdcdescr in interfacedescr.cdcs.iter() {
                    source.push(cdcdescr.source());
                }
                for endpointdescr in interfacedescr.endpoints.iter() {
                    source.push(endpointdescr.source());
                }
            }
            source.push("\n    ];".into());
        }

        source.push(self.string0.source());
        for stringdescr in self.strings.iter() {
            source.push(stringdescr.source());
        }
        source.push("\n    pub fn get_str(strdescr_id : u8) -> &'static [u8] {".into());
        source.push("\n        match strdescr_id { ".into());
        source.push("\n            0 => STRINGZERODESCRIPTOR,".into());
        for stringdescr in self.strings.iter() {
            source.push(format!("\n            {} => STRING_{}_DESCRIPTOR,", stringdescr.id, stringdescr.id));
        }
        source.push(format!("\n            _ => STRING_{}_DESCRIPTOR,", self.miss));
        source.push("\n        }\n    }\n".into());

        source.push(self.source_buffer_descriptor_table());

        source.push(self.endpoint_config().unwrap());// todo print error msg
        source
    }

    fn register_string(strings : &mut Vec<StringDescriptor>, i_string : &mut StringId) {
        if strings.len() > 254 {
            panic!("UsbDescriptor : too much strings");
        }
        let newdescr = StringDescriptor::new(&i_string.1, (strings.len() + 1) as u8);
        i_string.0 = newdescr.id;
        strings.push(newdescr);
    }

    fn collect_strings_calc_len(&mut self) {
        self.strings.clear();
        let mut strings : Vec<StringDescriptor> = vec![];
        Self::register_string(&mut strings, &mut self.device.iManufacturer);
        Self::register_string(&mut strings, &mut self.device.iProduct);
        Self::register_string(&mut strings, &mut self.device.iSerialNumber);


        for configdescr in self.device.configurations.iter_mut() {
            Self::register_string(&mut strings, &mut configdescr.iConfiguration);
            let mut length = 0u16; // length without device descriptor
            length += configdescr.len() as u16;

            for interfacedescr in configdescr.interfaces.iter_mut() {
                Self::register_string(&mut strings, &mut interfacedescr.iInterface);
                length += interfacedescr.len() as u16;

                for endpointdescr in interfacedescr.endpoints.iter() {
                    length += endpointdescr.len() as u16;
                }
                for cdcdescr in interfacedescr.cdcs.iter() {
                    length += cdcdescr.len() as u16;
                }
            }

            configdescr.wTotalLength = length;
        }

        let mut miss = StringId::new("No str fnd.");
        Self::register_string(&mut strings, &mut miss);
        self.miss = miss.0;

        self.strings = strings;
    }

    fn max_endpoint_addr(&self) -> usize {
        let mut max_addr = 0;
        if let Some(configdescr) = self.device.configurations.iter().next() {
            for interfacedescr in configdescr.interfaces.iter() {
                for endpointdescr in interfacedescr.endpoints.iter() {
                    max_addr = max(max_addr, endpointdescr.bEndpointAddress.addr() as usize);
                }
            }
        }
        max_addr
    }

    fn num_bufferdescriptors(&self) -> usize {
        (self.max_endpoint_addr() + 1) * 4
    }

    fn source_buffer_descriptor_table(&self) -> String {
        let num_bds = self.num_bufferdescriptors();
        let max_addr = self.max_endpoint_addr();
        let mut s = String::with_capacity(200);
        writeln!(s, "\n    pub const MAX_ENDPOINT_ADDR : u8 = {};" , max_addr).unwrap();
        writeln!(s, "\n    pub const NUM_BUFFERDESCRIPTORS : usize = {};" , num_bds).unwrap();
        writeln!(s, r#"
    extern {{
        #[no_mangle]
        #[link_name="_usbbufferdescriptors"]
        static mut usbbufferdescriptors : [::usb::BufferDescriptor; {}];
    }}

    #[allow(non_snake_case, dead_code)]
    #[inline(always)]
    pub fn BufferDescriptors() -> &'static mut [::usb::BufferDescriptor; {}] {{
        unsafe {{ &mut usbbufferdescriptors }}
    }}

        "# , num_bds, num_bds).unwrap();

        s
    }

    pub fn usbbufferdescriptors_size(&self) -> usize {
        self.num_bufferdescriptors() * 8
    }

    pub fn endpoint_config(&self) -> Result<String, String> {
        let mut source = String::with_capacity(1000);
        if let Some(configdescr) = self.device.configurations.iter().next() {
            let mut endp_configs = [Endpointconfig::default(); 16];
            for interfacedescr in configdescr.interfaces.iter() {
                for endpointdescr in interfacedescr.endpoints.iter() {
                    let endp_nr = endpointdescr.bEndpointAddress.addr() as usize;
                    let cfg : &mut Endpointconfig = &mut endp_configs[endp_nr];
                    match endpointdescr.bEndpointAddress {
                        EndpointAddress::In(_) => {
                            if cfg.rx() {
                                return Err(format!("Endpoint {} described as IN more than once.", endp_nr));
                            } else {
                                cfg.set_rx();
                            }
                        },
                        EndpointAddress::Out(_) => {
                            if cfg.tx() {
                                return Err(format!("Endpoint {} described as OUT more than once.", endp_nr));
                            } else {
                                cfg.set_tx();
                            }
                        },
                    }
                    if configdescr.interfaces.iter()
                       .flat_map(|i| i.endpoints.iter())
                       .filter(|e| e.bEndpointAddress.addr() as usize == endp_nr)
                       .any(|e| e.bmAttributes.is_isochronus() != endpointdescr.bmAttributes.is_isochronus()) {
                        // The usb standard maybe? allows this but the k20 usb circit does not support it
                        return Err(format!("Endpoint {} is configured as isochronous and as not isochronous.", endp_nr));
                    }
                    if !endpointdescr.bmAttributes.is_isochronus() {
                        cfg.set_notisochronous();
                    }
                }
            }
            source.push_str("\n    pub const ENDPOINTCONFIG_FOR_REGISTERS: &'static [u8] = &[\n\t\t");
            for cfg in (&endp_configs[..]).iter() {
                write!(source, " 0x{:02x},", cfg.0).unwrap();
            }
            source.push_str("\n    ];");
        }
        Ok(source)
    }
}

