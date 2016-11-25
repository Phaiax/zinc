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

use super::usbdescriptors::{DescriptorTree};
use builder::BuilderConfig;
use super::builder::MemoryConfig;
use util::Bytes;

pub struct UsbConfig {
    descriptortree : DescriptorTree,
}

impl UsbConfig {
    pub fn new(descriptortree : DescriptorTree, ) -> UsbConfig {
        UsbConfig {
            descriptortree : descriptortree,
        }
    }

    pub fn configure(&mut self, memory_config : &mut MemoryConfig) {
        memory_config.usbbufferdescriptors_size = Bytes::b(self.descriptortree.usbbufferdescriptors_size() as u32);
    }

    pub fn execute(&mut self, base_config : &mut BuilderConfig) {
        let mut source = self.descriptortree.source();
        for p in source.drain(..) {
            base_config.add_src(p);
        }

        base_config.add_src(r#"

    use usbmempool::{MemoryPool, UsbPacket};

    static mut POOL : Option<MemoryPool<[UsbPacket; 32]>> = None;
    pub fn pool_ref() -> &'static MemoryPool<[UsbPacket; 32]> {
        let r = unsafe { &mut POOL };
        if r.is_none() {
            *r = Some(MemoryPool::new());
        }
        &r.as_ref().unwrap()
    }

    use usbserial::UsbSerial;
    use usbdriver::{UsbDriver, DescriptorsAndMore};

    static mut USBDRIVER : Option<UsbSerial> = None;

    pub fn usb_ref() -> &'static mut UsbSerial {
        let r = unsafe { &mut USBDRIVER };
        if r.is_none() {
            *r = Some(UsbSerial::new(
                UsbDriver::new(pool_ref(),
                               BufferDescriptors(),
                               DescriptorsAndMore {
                                    devicedescriptor: DEVICEDESCRIPTOR,
                                    configdescriptortree: CONFIGDESCRIPTORTREE,
                                    get_str: get_str,
                                    endpointconfig_for_registers: ENDPOINTCONFIG_FOR_REGISTERS,
                               },
                               )));
        }
        r.as_mut().unwrap()
    }

    #[allow(dead_code)]
    #[no_mangle]
    pub unsafe extern "C" fn isr_usb() {
        usb_ref().isr();
    }

"#.into());

    }
}
