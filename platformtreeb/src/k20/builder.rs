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

use builder::{BuilderConfig};
use super::usb::UsbConfig;
use super::linkerscript::create_linker_script;
use super::types::McuParameters;
use util::{Bytes, write};
use std::path::PathBuf;
use std::env;

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

pub struct HardwareConfig {

}

pub struct McuSpecificConfig {
    base_config : BuilderConfig,
    mcu_parameters : McuParameters,
    memory_config : MemoryConfig,
    #[allow(dead_code)]
    hardware_config : HardwareConfig,
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
            hardware_config : HardwareConfig {

            },
            usb : None,
            linker_script : true,
        }
    }

    pub fn get_base_config_mut(&mut self) -> &mut BuilderConfig { &mut self.base_config }
    pub fn get_base_config(&self) -> &BuilderConfig { &self.base_config }
    pub fn get_mcu_parameters(&self) -> &McuParameters { &self.mcu_parameters }
    pub fn get_memory_config(&self) -> &MemoryConfig { &self.memory_config }
    pub fn get_memory_config_mut(&mut self) -> &mut MemoryConfig { &mut self.memory_config }
    pub fn get_e_flash_size(&self) -> Bytes {
        Bytes::b(self.mcu_parameters.get_flexnvm_size().0
         - self.memory_config.eeprom_backup_size.0)
    }

    pub fn with<F>(mut self, config_function : F) -> Self
        where F : FnOnce(&mut McuSpecificConfig) {
        config_function(&mut self);
        self
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
        self.base_config.add_src("// init for EEPROM".into());
        self
    }

    pub fn enable_usb(mut self, usb_config : UsbConfig) -> Self {
        self.usb = Some(usb_config);
        self
    }

    pub fn execute(&mut self) {
        println!("Execute Specific config");
        if self.linker_script {
            let mut out_dir : PathBuf = env::var("OUT_DIR").unwrap().into();
            println!("cargo:rustc-link-search=native={}", out_dir.to_str().unwrap());
            out_dir.push("layout.ld");
            write(&create_linker_script(&self), out_dir).unwrap();
        }
        if let Some(usb) = self.usb.as_mut() {
            usb.configure(&mut self.memory_config);
            usb.execute(&mut self.base_config);
        }
        self.base_config.execute();
    }
}

