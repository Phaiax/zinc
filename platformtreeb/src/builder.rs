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

    pub fn add_to_init(&mut self, part : InitPart, code : String) {
        if let Some(ref mut init_function) = self.init_function {
            init_function.add(part, code);
        }
    }

    pub fn execute(&mut self) {
        self.executed = true;
        if self.cargo_config {
            println!("Write cargo config");
            write(self.target.cargo_config(), ".cargo/config").unwrap();
        }
        if let Some(ref mod_and_name) = self.user_startup_function {
            self.src_generated_rs.push(format!("\nuse {} as user_entry_function;\n", mod_and_name));

            self.src_generated_rs.push(r#"
            #[start]
            fn generated_start(_: isize, _: *const *const u8) -> isize {"#.into());
            if self.init_function.is_some() {
                self.src_generated_rs.push("\n\t\t\t\tstartup();".into());
            }
            self.src_generated_rs.push(r#"
                user_entry_function();
                0
            }
            "#.into());
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

pub enum InitPart {
    UseStatement,
    Pre,
    Main,
    Post
}

pub struct InitFunction{
    usestatements : Vec<String>,
    function_pre : Vec<String>,
    function_main : Vec<String>,
    function_post : Vec<String>,
}

impl InitFunction {
    pub fn empty() -> InitFunction {
        InitFunction {
            usestatements : vec![],
            function_pre : vec![],
            function_main : vec![],
            function_post : vec![],
        }
    }

    pub fn default() -> InitFunction {
        let mut new = Self::empty();
        new.add(InitPart::Pre, r#"
        mem_init::init_stack();
        mem_init::init_data();
        "#.into());
        new.add(InitPart::UseStatement, "\n\nuse zinc::hal::mem_init;".into());
        new
    }

    pub fn add(&mut self, part : InitPart, code : String) {
        match part {
            InitPart::UseStatement => self.usestatements.push(code),
            InitPart::Pre => self.function_pre.push(code),
            InitPart::Main => self.function_main.push(code),
            InitPart::Post => self.function_post.push(code),
        }
    }

    fn drain(&mut self) -> Vec<String> {
        let mut combined = vec![];
        combined.append(&mut self.usestatements);
        combined.push(FUNCTION_HEADER.into());
        combined.append(&mut self.function_pre);
        combined.append(&mut self.function_main);
        combined.append(&mut self.function_post);
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
