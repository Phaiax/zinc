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

//! Mcu specific config

use builder::{BuilderConfig, McuType};
use super::usb::UsbConfig;
use super::linkerscript::create_linker_script;
use super::types::McuParameters;
use util::{Bytes, write};

/// Some Microcontroller allow splitting the EEPROM 
/// to allow two different endurance vs size tradeoffs
/// at the same time.
#[derive(Copy,Clone)]
pub enum EEESplit {
    Half,
    Quarter,
    Eighth,
    Unsupported,
}

impl EEESplit {
    pub fn is_unsupported(&self) -> bool {
        match self {
            &EEESplit::Unsupported => true,
            _ => false
        }
    }
}

pub struct MemoryConfig {
    pub eeprom_size : Bytes,
    pub eeprom_split : EEESplit,
    pub eeprom_backup_size : Bytes,
    pub usbdescriptortable_size : Bytes,
}

pub struct McuSpecificConfig {
    base_config : BuilderConfig,
    mcu_parameters : McuParameters,
    memory_config : MemoryConfig,
    usb : Option<UsbConfig>,
    linker_script : bool,
}

impl McuSpecificConfig {
    pub fn new(m : BuilderConfig) -> McuSpecificConfig {
        let mcu_parameters = m.get_mcu_type().into();
        McuSpecificConfig {
            base_config : m,
            mcu_parameters : mcu_parameters,
            memory_config : MemoryConfig {
                eeprom_size : Bytes::b(0),
                eeprom_split : EEESplit::Unsupported,
                eeprom_backup_size : Bytes::b(0),
                usbdescriptortable_size : Bytes::b(0),
            },
            usb : None,
            linker_script : true,
        }
    }

    pub fn get_base_config(&self) -> &BuilderConfig { &self.base_config }
    pub fn get_mcu_parameters(&self) -> &McuParameters { &self.mcu_parameters }
    pub fn get_memory_config(&self) -> &MemoryConfig { &self.memory_config }
    pub fn get_e_flash_size(&self) -> Bytes {
        Bytes::b(self.mcu_parameters.get_flexnvm_size().0
         - self.memory_config.eeprom_backup_size.0)
    }

    pub fn set_eeprom(mut self, eeprom_size : Bytes,
                      eeprom_backup_size : Bytes,
                      eeprom_split : EEESplit) -> Self {
        self.memory_config.eeprom_size = eeprom_size;
        self.memory_config.eeprom_backup_size = eeprom_backup_size; // set before call to get_e_flash_size()
        self.memory_config.eeprom_split = eeprom_split;
        if !self.mcu_parameters.supports_eeprom_split() && !eeprom_split.is_unsupported()  {
            panic!("Chosen MCU does not support EEESplit. You must use EEESplit::Unsupported.");
        }
        if self.mcu_parameters.supports_eeprom_size(eeprom_size).is_none() {
            println!("Supported EEPROM sizes: {}", self.mcu_parameters.supported_eeprom_sizes());
            panic!("Chosen EEPRROM size is unsupported.");
        }
        if self.mcu_parameters.supports_flexnvm_partition_size(self.get_e_flash_size()).is_none() {
            println!("Supported EEPROM backup sizes: {}",
                     self.mcu_parameters.supported_eeprom_backup_sizes());
            panic!("Chosen EEPRROM backup size is unsupported.");
        }
        self
    }

    fn execute(&mut self) {
        if self.linker_script {
            write(&create_linker_script(&self), "Layout.ld").unwrap();
        }
        println!("Execute Specific config");
    }
}


impl Drop for McuSpecificConfig {
    fn drop(&mut self) {
        self.execute();
    }
}