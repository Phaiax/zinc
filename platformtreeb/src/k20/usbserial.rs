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

use super::usbdescriptors::{DescriptorTree, ConfigurationDescriptor, DeviceDescriptor,
    DeviceClass, MaxPacketSizeEP0, Bcd, StringId, ConfAttributes, MaxCurrent, BcdUsb,
    InterfaceDescriptor, EndpointDescriptor, EndpointAddress,
    TransferType, CdcInterfaceFunctionalDescriptor, BcdCdc};


pub fn make_device_tree_for_teensy_serial() -> DescriptorTree {
    let d = DeviceDescriptor {
        bcdUSB : BcdUsb::Usb11,
        bDeviceClass : DeviceClass::ClassCode(2), // Communications Device Class
        bDeviceSubClass : 0, // Communications Device Subclass code, unused at this time.
        bDeviceProtocol : 0, // Communications Device Protocol code, unused at this time.
        bMaxPacketSize0 : MaxPacketSizeEP0::Bytes64,
        idVendor : 0x16C0,
        idProduct : 0x0483,
        bcdDevice : Bcd(0x0100),
        iManufacturer : StringId::new("Daniel"),
        iProduct :  StringId::new("THE Profud"),
        iSerialNumber : StringId::new("12345"),
        configurations : vec![ConfigurationDescriptor {
            wTotalLength : 0, // will be updated automatically
            bConfigurationValue : 1,
            iConfiguration : StringId::new("Blub"),
            bmAttributes : ConfAttributes {
                self_powered : true,
                remote_wakeup : false,
            },
            bMaxPower : MaxCurrent {
                milli_amps : 100
            },
            interfaces : vec![
                InterfaceDescriptor { // Communications Class Interface
                    bInterfaceNumber : 0,
                    bAlternateSetting : 0,
                    bInterfaceClass : 0x02, // Communications Interface Class
                    bInterfaceSubClass : 0x02, // Communications Class Subclass: Abstract Control Model
                    bInterfaceProtocol : 0x01, // Communications Interface Class Control Protocol: AT Commands: V.250 etc
                    iInterface : StringId::new("Int1"),
                    cdcs : vec![
                        CdcInterfaceFunctionalDescriptor::Header { bcdCDC : BcdCdc::Cdc12 },
                        CdcInterfaceFunctionalDescriptor::CallManagement { bmCapabilities : 0, bDataInterface : 1 },
                        CdcInterfaceFunctionalDescriptor::AbstractControlManagement { bmCapabilities : 0x06 },
                        CdcInterfaceFunctionalDescriptor::Union { bControlInterface : 0, bSubordinateInterfaces : vec![1] },
                    ],
                    endpoints : vec![
                        EndpointDescriptor {
                            bEndpointAddress : EndpointAddress::In(2),
                            bmAttributes : TransferType::Interrupt,
                            wMaxPacketSize : 16,
                            bInterval : 64,
                        },
                    ],
                },
                InterfaceDescriptor { // Data Class Interface
                    bInterfaceNumber : 1,
                    bAlternateSetting : 0,
                    bInterfaceClass : 0x0A, // Data Interface Class code
                    bInterfaceSubClass : 0x00, // At this time this field is un-used for Data Class interfaces and should have a value of 00h.
                    bInterfaceProtocol : 0x00, // Data Interface Class Protocol: No class specific protocol required
                    iInterface : StringId::new("Int2"),
                    cdcs : vec![],
                    endpoints : vec![
                        EndpointDescriptor {
                            bEndpointAddress : EndpointAddress::Out(3),
                            bmAttributes : TransferType::Bulk,
                            wMaxPacketSize : 64,
                            bInterval : 0,
                        },
                        EndpointDescriptor {
                            bEndpointAddress : EndpointAddress::In(4),
                            bmAttributes : TransferType::Bulk,
                            wMaxPacketSize : 64,
                            bInterval : 0,
                        },
                    ],

                },
            ],
        }],
    };
    DescriptorTree::new(d)
}
