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
    pub fn cargo_config(&self) -> &'static str {
        match self {
            &Target::Thumbv6m => CARGOCONFIG_THUMBV6M,
            &Target::Thumbv7em => CARGOCONFIG_THUMBV7EM,
            &Target::Thumbv7m => CARGOCONFIG_THUMBV7M,
        }
    }
}

const CARGOCONFIG_THUMBV7M: &'static str = r#"
[build]
target = "thumbv7m-none-eabi"

[target.thumbv7m-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
rustflags = [
    "-C",
    "link-arg=-Tlayout.ld",
]
"#;

const CARGOCONFIG_THUMBV7EM: &'static str = r#"
[build]
target = "thumbv7em-none-eabi"

[target.thumbv7em-none-eabi]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
rustflags = [
    "-C",
    "link-arg=-Tlayout.ld",
]
"#;

const CARGOCONFIG_THUMBV6M: &'static str = r#"
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
rustflags = [
    "-C",
    "link-arg=-Tlayout.ld",
]
linker = "arm-none-eabi-gcc"
ar = "arm-none-eabi-ar"
"#;
