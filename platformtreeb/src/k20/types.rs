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

use builder::McuType;
use util::Bytes;

/// This is the static configuration of the used chip
/// The actual used eeprom size will not be configured here.
pub struct McuParameters {
    /// Also called P-Flash
    program_flash_size : Bytes,
    /// Second bank of flash memory, called E-Flash or FlexNVM. (NVM : Non volatile memory)
    /// This part can be used as program flash. Alternatively, it can be partially
    /// used as a backend for EEPROM. Then the FlexRam functions as an interface
    /// for the EEPROM data, and the chip internals write the data to the FlexNVM.
    /// The endurance of the EEPROM can be adjusted by the ratio of the assigned
    /// memory ranges. See application note
    /// [AN4282](http://cache.freescale.com/files/32bit/doc/app_note/AN4282.pdf)
    /// TODO: Move this documentation to a more visible place.
    flexnvm_size : Bytes,
    flexram_eeprom_size : Bytes,
    /// Possible configurations for eeprom sizes. The u8 value is the bit pattern
    /// that must be written into EEESIZE to enable that configuration.
    eeprom_size_configurations : Vec<(Bytes, u8)>,
    support_eeprom_split : bool,
    /// Possible configurations for FlexNVM partition. The u8 value is the bit pattern
    /// that must be written into DEPART to enable that configuration.
    /// The Bytes value defines the size of the part that is used for EEPROM backup.
    /// The size of the still usable data part is flexnvm_size - (the Bytes value)
    flexnvm_partition_configurations : Vec<(Bytes, u8)>,   
    sram_size : Bytes,
}



impl McuParameters {
    pub fn get_program_flash_size(&self) -> Bytes { self.program_flash_size }
    pub fn get_flexnvm_size(&self) -> Bytes { self.flexnvm_size }
    pub fn get_flexram_eeprom_size(&self) -> Bytes { self.flexram_eeprom_size }
    pub fn get_sram_size(&self) -> Bytes { self.sram_size }
    pub fn supports_eeprom_split(&self) -> bool { self.support_eeprom_split }
    pub fn supports_eeprom_size(&self, eeprom_size : Bytes) -> Option<u8> {
        self.eeprom_size_configurations.iter()
            .find(|&&(sz, _)| sz.0 == eeprom_size.0 )
            .map(|&(_, pattern)| pattern)
    }
    pub fn supports_flexnvm_partition_size(&self, flexnvm_size : Bytes) -> Option<u8> {
        self.flexnvm_partition_configurations.iter()
            .find(|&&(sz, _)| sz.0 == flexnvm_size.0 )
            .map(|&(_, pattern)| pattern)
    }
    pub fn supported_eeprom_sizes(&self) -> String {
        self.eeprom_size_configurations.iter()
            .fold(String::new(), |mut s, conf| { s.push_str(&format!("{}, ", &conf.0.str())); s } )

    }
    pub fn supported_flexnvm_partition_sizes(&self) -> String {
        self.flexnvm_partition_configurations.iter()
            .fold(String::new(), |mut s, conf| { s.push_str(&format!("{}, ", &conf.0.str())); s } )
    }
    pub fn supported_eeprom_backup_sizes(&self) -> String {
        self.flexnvm_partition_configurations.iter()
            .fold(String::new(), |mut s, conf| { 
                s.push_str(&format!("{}, ", &(self.flexnvm_size - conf.0).str())); s } )
    }
}

impl From<McuType> for McuParameters {
    fn from(t : McuType) -> McuParameters {
        match t {
            McuType::Mk20dx256vlh7 => McuParameters {
                program_flash_size : Bytes::k(256),
                flexnvm_size : Bytes::k(32),
                flexram_eeprom_size : Bytes::k(2),
                eeprom_size_configurations : vec![
                    (Bytes::k(2), 3), (Bytes::k(1), 4), (Bytes::b(512), 5), (Bytes::b(256), 6),
                    (Bytes::b(128), 7), (Bytes::b(64), 7), (Bytes::b(32), 9), (Bytes::b(0), 15)],
                support_eeprom_split : false,
                flexnvm_partition_configurations : vec![(Bytes::k(0), 0), (Bytes::k(8), 1), 
                    (Bytes::k(16), 2), (Bytes::k(24), 9), (Bytes::k(32), 8)],
                sram_size : Bytes::k(64),
            },
            _ => panic!(format!("MCU type {:?} is not implemented. See src/k20/types.rs.", t))
        }

    }
}