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
use util::{write, write_vec};
use std::borrow::Cow;
use std::collections::HashMap;

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

    cargo_config : bool,
    user_startup_function : Option<String>,

    init_function : Option<InitFunction>,
    src_generated_rs : Vec<String>,

    called_mcu : bool,
    executed : bool,
}



impl BuilderConfig {
    pub fn new(mcu_class : McuClass, mcu_type : McuType) -> BuilderConfig {
        BuilderConfig {
            mcu_class : mcu_class,
            mcu_type : mcu_type,
            target : get_target(),

            cargo_config : true,
            user_startup_function : None,

            init_function : Some(InitFunction::default()),
            src_generated_rs : vec!["#![allow(dead_code)]\n".into()],

            called_mcu : false,
            executed : false,
        }
    }

    pub fn get_mcu_class(&self) -> McuClass { self.mcu_class }
    pub fn get_mcu_type(&self) -> McuType { self.mcu_type }
    pub fn get_target(&self) -> Target { self.target }

    pub fn skip_adding_cargo_config(mut self) -> BuilderConfig {
        self.cargo_config = false;
        self
    }

    /// Inserts a reset entry point (= `#[start] fn main(_,_);` ).
    /// First calls the generated initialization routine (if any),
    /// then calls the user statup routine `mod_and_name()`.
    /// This requires you to add `#![feature(start)]` for your crate.
    pub fn call_user_startup_function(mut self, mod_and_name : &str) -> BuilderConfig {
        self.user_startup_function = Some(mod_and_name.into());
        self
    }

    pub fn mcu(mut self) -> McuSpecificConfig {
        self.called_mcu = true;
        McuSpecificConfig::new(self)
    }

    pub fn add_src(&mut self, src : String) {
        self.src_generated_rs.push(src);
    }

    pub fn add_to_init<'a, T:Into<Cow<'a, str>>>(&mut self, part : InitPart, code : T) {
        if let Some(ref mut init_function) = self.init_function {
            init_function.add(part, code);
        }
    }

    pub fn usetype_in_init<'a, T:Into<Cow<'a, str>>>(&mut self, code : T) {
        if let Some(ref mut init_function) = self.init_function {
            init_function.usetype(code);
        }
    }

    pub fn execute(&mut self) {
        self.executed = true;
        if self.cargo_config {
            println!("Write cargo config");
            write(self.target.cargo_config(), ".cargo/config").unwrap();
        }
        if let Some(ref mod_and_name) = self.user_startup_function {
            self.src_generated_rs.push(format!("\n\n\tuse {} as user_entry_function;\n", mod_and_name));

            self.src_generated_rs.push("\n\t#[start]".into());
            self.src_generated_rs.push("\n\tfn generated_start(_: isize, _: *const *const u8) -> isize {".into());
            if self.init_function.is_some() {
                self.src_generated_rs.push("\n\t\tstartup();".into());
            }
            self.src_generated_rs.push("\n\t\tuser_entry_function();\n\t\t0\n\t}\n".into());
        }
        if let Some(ref mut init_function) = self.init_function {
            self.src_generated_rs.append(&mut init_function.drain());
        }
        if self.src_generated_rs.len() > 0 {
            write_vec(&self.src_generated_rs, "src/generated.rs").unwrap();
        }
    }

}

impl Drop for BuilderConfig {
    fn drop(&mut self) {
        if !self.called_mcu {
            panic!("You must call mcu() on the BuilderConfig instance in your build.rs file.")
        }
        if !self.executed {
            panic!("You must call execute() on the BuilderConfig or mcu() instance in your build.rs file.")
        }
    }
}

#[repr(usize)]
pub enum InitPart {
    /// e.g. Disable watchdog
    Pos00Earliest = 0,
    /// e.g. Init static memory
    Pos05Early = 5,
    /// Setup clocks
    Pos10Early = 10,
    /// Setup peripherals
    Pos40Main = 40,
    /// Setup other stuff
    Pos60Late = 60,
    /// Init drivers for user startup function
    Pos80Usr = 80,
}

pub struct InitFunction{
    usestatements : Vec<String>,
    code : HashMap<usize, Vec<String>>,
}

impl InitFunction {
    pub fn empty() -> InitFunction {
        InitFunction {
            usestatements : vec![],
            code : HashMap::new(),
        }
    }

    pub fn default() -> InitFunction {
        let mut new = Self::empty();
        new.add(InitPart::Pos05Early, r#"
        mem_init::init_stack();
        mem_init::init_data();
        "#);
        new.usetype("zinc::hal::mem_init");
        new
    }

    pub fn add<'a, T:Into<Cow<'a, str>>>(&mut self, part : InitPart, code : T) {
        self.code.entry(part as usize).or_insert_with(|| vec![]).push(code.into().into_owned());
    }

    pub fn usetype<'a, T:Into<Cow<'a, str>>>(&mut self, type_ : T) {
        self.usestatements.push(format!("\nuse {};", type_.into()));
    }

    fn drain(&mut self) -> Vec<String> {
        let mut combined = vec![];
        combined.append(&mut self.usestatements);
        combined.push(FUNCTION_HEADER.into());
        for i in 0..99usize {
            match self.code.get_mut(&i) {
                Some(mut vec) => {
                    combined.append(&mut vec);
                },
                _ => {}
            }
        }
        combined.push(FUNCTION_FOOTER.into());
        combined
    }
}

const FUNCTION_HEADER : &'static str =   r#"

#[inline(always)]
pub fn startup() {
"#;

const FUNCTION_FOOTER : &'static str =   r#"
}

"#;
