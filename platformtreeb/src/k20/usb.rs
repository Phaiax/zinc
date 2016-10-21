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
    pub fn new(descriptortree : DescriptorTree) -> UsbConfig {
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
    }
}
