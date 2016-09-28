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

#[derive(Clone, Copy)]
pub enum Target {
    Thumbv6m,
    Thumbv7em,
    Thumbv7m,
}

impl Target {
    pub fn target_json_filename(&self) -> &'static str {
        match self {
            &Target::Thumbv6m => "thumbv6m-none-eabi.json",
            &Target::Thumbv7em => "thumbv7em-none-eabi.json",
            &Target::Thumbv7m => "thumbv7m-none-eabi.json",
        }
    }
    pub fn target_json(&self) -> &'static str {
        match self {
            &Target::Thumbv6m => THUMBV6M,
            &Target::Thumbv7em => THUMBV7EM,
            &Target::Thumbv7m => THUMBV7M,
        }
    }
    pub fn cargo_config(&self) -> &'static str {
        match self {
            &Target::Thumbv6m => CARGOCONFIG_THUMBV6M,
            &Target::Thumbv7em => CARGOCONFIG_THUMBV7EM,
            &Target::Thumbv7m => CARGOCONFIG_THUMBV7M,
        }
    }
}

const THUMBV6M: &'static str = r#"
{
    "arch": "arm",
    "cpu": "cortex-m0",
    "data-layout": "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64",

    "env": "eabi",
    "executables": true,
    "llvm-target": "thumbv6m-none-eabi",
    
    "os": "none",
    "relocation-model": "static",
    "target-endian": "little",
    "target-pointer-width": "32",
    "no-compiler-rt": true
    "pre-link-args": [
        "-Tlayout.ld"
    ],
    "post-link-args": [
        "-lm", "-lgcc", "-lnosys"
    ],
    "vendor": "unknown",
}
"#;

const THUMBV7EM: &'static str = r#"
{
    "arch": "arm",
    "cpu": "cortex-m4",
    "data-layout": "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64",
    
    "disable-redzone": true,
    "executables": true,
    "llvm-target": "thumbv7em-none-eabi",
    "morestack": false,
    "os": "none",
    "relocation-model": "static",
    "target-endian": "little",
    "target-pointer-width": "32",
    "no-compiler-rt": true,
    "pre-link-args": [
        "-mcpu=cortex-m4", "-mthumb",
        "-Tlayout.ld"
    ],
    "post-link-args": [
        "-lm", "-lgcc", "-lnosys"
    ]
}
"#;

const THUMBV7M: &'static str = r#"
{
    "arch": "arm",
    "cpu": "cortex-m3",
    "data-layout": "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64",

    "disable-redzone": true,
    "executables": true,
    "llvm-target": "thumbv7m-none-eabi",
    "morestack": false,
    "os": "none",
    "relocation-model": "static",
    "target-endian": "little",
    "target-pointer-width": "32",
    "no-compiler-rt": true,
    "pre-link-args": [
        "-mcpu=cortex-m3", "-mthumb",
        "-Tlayout.ld"
    ],
    "post-link-args": [
        "-lm", "-lgcc", "-lnosys"
    ]
}
"#;

const CARGOCONFIG_THUMBV7M: &'static str = r#"
[build]
target = "thumbv7m-none-eabi"

[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
"#;

const CARGOCONFIG_THUMBV7EM: &'static str = r#"
[build]
target = "thumbv7em-none-eabi"

[target.thumbv7em-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
"#;

const CARGOCONFIG_THUMBV6M: &'static str = r#"
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
"#;
