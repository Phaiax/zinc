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
    TransferType, CDCDescriptor, CDCDescriptorSubTypes};


pub fn make_device_tree_for_teensy_serial() -> DescriptorTree {
    let d = DeviceDescriptor {
        bcdUSB : BcdUsb::Usb20,
        bDeviceClass : DeviceClass::ClassCode(2),
        bDeviceSubClass : 0,
        bDeviceProtocol : 0,
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
                InterfaceDescriptor {
                    bInterfaceNumber : 0,
                    bAlternateSetting : 0,
                    bInterfaceClass : 0x02,
                    bInterfaceSubClass : 0x02,
                    bInterfaceProtocol : 0x01,
                    iInterface : StringId::new("Int1"),
                    cdcs : vec![
                        CDCDescriptor {
                            bDescriptorSubType : CDCDescriptorSubTypes::CDCHeader,
                            bytes : vec![0x10, 0x01], // bcdCDC
                        },
                        CDCDescriptor {
                            bDescriptorSubType : CDCDescriptorSubTypes::CallManagement,
                            bytes : vec![0x00, 0x01], // [bmCapabilities, bDataInterface]
                        },
                        CDCDescriptor {
                            bDescriptorSubType : CDCDescriptorSubTypes::AbstactControl,
                            bytes : vec![0x06], // bmCapabilities?
                        },
                        CDCDescriptor {
                            bDescriptorSubType : CDCDescriptorSubTypes::UnionFunctional,
                            bytes : vec![0, 1], // [bMasterInterface, bSlaveInterface0]
                        }

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
                InterfaceDescriptor {
                    bInterfaceNumber : 1,
                    bAlternateSetting : 0,
                    bInterfaceClass : 0x0A,
                    bInterfaceSubClass : 0x00,
                    bInterfaceProtocol : 0x00,
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
