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

use mcu::{McuSpecificConfig, get_target};
use target::Target;
use util::write;

#[derive(Clone, Copy)]
pub enum McuClass {
    K20,
}

#[derive(Clone, Copy, Debug)]
pub enum McuType {
    Mk20dx256vlh7,
    Unimplemented,
}

pub struct BuilderConfig {
    mcu_class : McuClass,
    mcu_type : McuType,
    target : Target,

    target_json : bool,
    cargo_config : bool,

    called_mcu : bool,
}



impl BuilderConfig {
    pub fn new(mcu_class : McuClass, mcu_type : McuType) -> BuilderConfig {
        BuilderConfig {
            mcu_class : mcu_class,
            mcu_type : mcu_type,
            target : get_target(),

            target_json : true,
            cargo_config : true,

            called_mcu : false,
        }
    }

    pub fn get_mcu_class(&self) -> McuClass { self.mcu_class }
    pub fn get_mcu_type(&self) -> McuType { self.mcu_type }
    pub fn get_target(&self) -> Target { self.target }

    pub fn skip_adding_target_json(mut self) -> BuilderConfig {
        self.target_json = false;
        self
    }

    pub fn skip_adding_cargo_config(mut self) -> BuilderConfig {
        self.cargo_config = false;
        self
    }

    pub fn mcu(mut self) -> McuSpecificConfig {
        self.called_mcu = true;
        McuSpecificConfig::new(self)
    }

    fn execute(&mut self) {
        if self.target_json {
            println!("Write target_json");
            write(self.target.target_json(), self.target.target_json_filename()).unwrap();
        }
        if self.cargo_config {
            println!("Write cargo config");
            write(self.target.cargo_config(), ".cargo/config").unwrap();
        }
    }

}

impl Drop for BuilderConfig {
    fn drop(&mut self) {
        if !self.called_mcu {
            panic!("You must call mcu() on the BuilderConfig instance in your build.rs file.")
        }
        self.execute();
    }
}