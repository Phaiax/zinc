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

pub mod util;
pub mod builder;
pub mod mcu;
pub mod target;

#[cfg(feature = "mcu_k20")] pub mod k20;

pub use builder::McuType;
pub use util::Bytes;

#[cfg(feature = "mcu_k20")]
pub fn start(mcu : McuType) -> builder::BuilderConfig {
    builder::BuilderConfig::new(builder::McuClass::K20, mcu)
}